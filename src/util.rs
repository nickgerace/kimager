/*
 * image-logger
 * https://github.com/nickgerace/image-logger
 * Author: Nick Gerace
 * License: Apache 2.0
 */

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use k8s_openapi::api::core::v1::Pod;
use log::debug;

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

pub fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn hash_tuple(t: &(&str, &str)) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}
