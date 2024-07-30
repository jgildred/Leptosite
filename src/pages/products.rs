// see below for customization.
// to create a new page, copy this file, rename and modify.

use leptos::*;
use crate::footer;

#[component]
pub fn Section(
    currpage: ReadSignal<i32>,
    set_currpage: WriteSignal<i32>
) -> impl IntoView {
    let page_id = 1; // this must be unique
    view! {
    // customize your page here with divs and associated style classes.
    // define additional style classes in styles.css.

    <div class="page" 
        style:visibility = move || 
            if page_id == currpage() { "visible" }
            else { "hidden" }>
        <div class="page_row_spacer" style="min-height:100px;"></div>
        <div class="page_row_title">
            Lorem ipsum
        </div>
        <div class="page_row">
            <img class="page_row_item rounded_box" src="images/prod_watermelon.png"></img>
            <p class="page_row_item page_text">
                Lorem ipsum dolor sit amet, consectetur adipiscing 
                elit, sed do eiusmod tempor incididunt ut labore et 
                dolore magna aliqua. Ut enim ad minim veniam, quis 
                nostrud exercitation ullamco laboris nisi ut aliquip 
                ex ea commodo consequat. Duis aute irure dolor in 
                reprehenderit in voluptate velit esse cillum dolore 
                eu fugiat nulla pariatur.
            </p>
        </div>
        <div class="page_row_spacer" style="min-height:20px;"></div>
        <div class="page_row_title">
            Duis aute
        </div>
        <div class="page_row">     
            <p class="page_row_item page_text">
                Lorem ipsum dolor sit amet, consectetur adipiscing 
                elit, sed do eiusmod tempor incididunt ut labore et 
                dolore magna aliqua. Ut enim ad minim veniam, quis 
                nostrud exercitation ullamco laboris nisi ut aliquip 
                ex ea commodo consequat. Duis aute irure dolor in 
                reprehenderit in voluptate velit esse cillum dolore 
                eu fugiat nulla pariatur.
            </p>
            <img class="page_row_item rounded_box" src="/images/prod_pomegranate.png"></img>
        </div>
        <div class="page_row_spacer" style="min-height:20px;"></div>
        <div class="page_row_title">
            Lorem ipsum
        </div>
        <div class="page_row">
            <img class="page_row_item rounded_box" src="images/prod_cherries.png"></img>
            <p class="page_row_item page_text">
                Lorem ipsum dolor sit amet, consectetur adipiscing 
                elit, sed do eiusmod tempor incididunt ut labore et 
                dolore magna aliqua. Ut enim ad minim veniam, quis 
                nostrud exercitation ullamco laboris nisi ut aliquip 
                ex ea commodo consequat. Duis aute irure dolor in 
                reprehenderit in voluptate velit esse cillum dolore 
                eu fugiat nulla pariatur.
            </p>
        </div>
        <div class="page_row_spacer" style="min-height:100px;"></div>
        <footer::Section page_id=page_id currpage=currpage set_currpage=set_currpage/>
    </div>

}}