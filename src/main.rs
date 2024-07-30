// this file does not need to be modified

use leptos::*;
mod web_site;
mod nav_menu;
mod footer;
mod pages;

fn main() {
    mount_to_body(|| view! { <web_site::Section/> });
}

 