use k8s_openapi::api::core::v1::Pod;
use log::debug;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub fn get_images(pod: &Pod) -> Option<Vec<String>> {
    match &pod.spec {
        Some(spec) => {
            let mut images: Vec<String> = Vec::new();
            for container in &spec.containers {
                match &container.image {
                    Some(image) => images.push(image.to_owned()),
                    None => debug!("Container {:#?} does not contain image", container.name),
                }
            }
            if images.is_empty() {
                return None;
            }
            Some(images)
        }
        None => None,
    }
}

pub fn hash_string(s: &str, hasher: &mut DefaultHasher) -> u64 {
    s.hash(hasher);
    hasher.finish()
}

pub fn hash_tuple(t: &(&str, &str), hasher: &mut DefaultHasher) -> u64 {
    t.hash(hasher);
    hasher.finish()
}
