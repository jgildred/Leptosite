// see below for customization.
// to create a new page, copy this file, rename and modify.

use leptos::*;
use crate::footer;

#[component]
pub fn Section(
    currpage: ReadSignal<i32>,
    set_currpage: WriteSignal<i32>
) -> impl IntoView {
    let page_id = 0; // this must be unique
    view! {
    // customize your page here with divs and associated style classes.
    // define additional style classes in styles.css.

    <div class="page home_page"
        style:visibility = move || 
            if page_id == currpage() { "visible" }
            else { "hidden" }>
        <div class="hero_title">Get Away and Relax.</div>
        <div class="hero_text">
            Lorem ipsum dolor sit amet, consectetur adipiscing 
            elit, sed do eiusmod tempor incididunt ut labore et 
            dolore magna aliqua.
        </div>
        <footer::Section page_id=page_id currpage=currpage set_currpage=set_currpage/> 
    </div>
    
}}