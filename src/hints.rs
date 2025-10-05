#![allow(unused)]

use std::sync::atomic::AtomicUsize;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use log::log;

pub struct Image;

impl Image {
    pub fn make_thumbnail(&self) -> Self {
        Self
    }
}

pub fn make_thumbnails_simple(images: &[Image]) -> Vec<Image> {
    let mut i = 0;
    let len = images.len();
    let mut output = Vec::with_capacity(len);
    while i < len {
        output.push(images[i].make_thumbnail());
        i += 1;
    }

    output
}

// Improved
pub fn make_thumbnails(images: &[Image]) -> Vec<Image> {
    images
        .iter()
        .map(|image| image.make_thumbnail())
        .collect()
}

// Multithreaded with Rayon
pub fn make_thumbnails_multithreaded(images: &[Image]) -> Vec<Image> {
    images
        .par_iter()
        .map(|image| image.make_thumbnail())
        .collect()
}

pub fn count_thumbnails_race_conditioned(images: &[Image]) -> Vec<Image> {
    let mut counter = 0;
    let vec = images
        .par_iter()
        .map(|image| {
            // counter += 1; 
            image.make_thumbnail()
        })
        .collect();
    println!("{}", counter);
    vec
}

pub fn count_thumbnails_safe(images: &[Image]) -> Vec<Image> {
    let counter = AtomicUsize::new(0);
    let vec = images
        .par_iter()
    .map(|image| {
        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        image.make_thumbnail()
    })
        .collect();
    println!("{}", counter.load(std::sync::atomic::Ordering::SeqCst));
    
    vec
}