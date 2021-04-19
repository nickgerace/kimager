use crate::util;
use bimap::BiMap;
use k8s_openapi::api::core::v1::Pod;
use kube::api::Resource;
use log::{debug, error, info};
use std::collections::{hash_map::DefaultHasher, HashMap};

pub enum EventType {
    Added,
    Deleted,
}

pub struct EventDriver {
    images: BiMap<String, u64>,
    pods: BiMap<(String, String), u64>,
    map: HashMap<u64, Vec<u64>>,
}

impl EventDriver {
    pub fn new() -> EventDriver {
        EventDriver {
            images: BiMap::new(),
            pods: BiMap::new(),
            map: HashMap::new(),
        }
    }

    pub async fn new_event(&mut self, pod: Pod, event: EventType) {
        let pod_name = Resource::name(&pod);
        match Resource::namespace(&pod) {
            Some(pod_ns) => match event {
                EventType::Added => {
                    debug!("[+|pod]  {}  [{}]", &pod_name, &pod_ns);
                    self.add_event(pod, pod_name, pod_ns).await;
                }
                EventType::Deleted => {
                    debug!("[-|pod]  {}  [{}]", &pod_name, &pod_ns);
                    self.delete_event(pod_name, pod_ns).await;
                }
            },
            None => error!("Could not get namespace for pod: {}", &pod_name),
        };
    }

    async fn add_event(&mut self, pod: Pod, pod_name: String, pod_ns: String) {
        match util::get_images(&pod) {
            Some(images) => {
                let mut hasher = DefaultHasher::new();
                let pod_hash = util::hash_tuple(&(&pod_name, &pod_ns), &mut hasher);
                self.pods
                    .insert((pod_name.clone(), pod_ns.clone()), pod_hash);
                for image in images {
                    let image_hash = util::hash_string(&image, &mut hasher);
                    self.images.insert(image.clone(), image_hash);
                    if !self.map.contains_key(&image_hash) {
                        info!("[+|image]  {}  [{}|{}]", &image, &pod_ns, &pod_name);
                    } else {
                        debug!("[+=|image]  {}  [{}|{}]", &image, &pod_ns, &pod_name);
                    }
                    let image_submap = self.map.entry(image_hash).or_default();
                    image_submap.push(pod_hash);
                    image_submap.sort();
                    image_submap.dedup();
                    let pod_submap = self.map.entry(pod_hash).or_default();
                    pod_submap.push(image_hash);
                    pod_submap.sort();
                    pod_submap.dedup();
                }
            }
            None => error!("Pod spec is empty: {:#?}", &pod_name),
        };
    }

    /// For every image hash found in the pod hash's corresponding vector, retain all pod hashes
    /// except for the pod being deleted. If there are no more pods being referenced, the image can
    /// be removed. Once all image hashes have the pod hash removed from their pod hash vectors, we
    /// can safely delete the pod hash entry. We can remove the pod from the index regardless of
    /// its existence.
    async fn delete_event(&mut self, pod_name: String, pod_ns: String) {
        let index = &(pod_name, pod_ns);
        if let Some(hash) = self.pods.get_by_left(index) {
            let pod_hash = hash.to_owned();

            let mut image_hashes: Vec<u64> = Vec::new();
            for image_hash in self.map.entry(pod_hash).or_default() {
                image_hashes.push(image_hash.to_owned());
            }

            for image_hash in image_hashes {
                let image_submap = self.map.entry(image_hash).or_default();
                image_submap.retain(|&x| x != pod_hash);
                match self.images.get_by_right(&image_hash) {
                    Some(image) => {
                        if image_submap.is_empty() {
                            info!("[-|image]  {}  [{}|{}]", image, index.1, index.0);
                        } else {
                            debug!("[-=|image]  {}  [{}|{}]", image, index.1, index.0);
                        }
                        self.images.remove_by_right(&image_hash);
                    }
                    None => error!("Could not find image for hash: {}", &image_hash),
                };
            }
            self.map.remove(hash);
        }
        self.pods.remove_by_left(index);
    }
}
