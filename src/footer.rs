use leptos::*;

#[component]
pub fn Section(
    page_id: i32,
    currpage: ReadSignal<i32>,
    set_currpage: WriteSignal<i32>
) -> impl IntoView {
    view! {
    // customize the footer layout below with divs and style classes.
    // modify or define new style classes in styles.css.

    <div class="footer" 
        style:visibility = move || 
            if page_id == currpage() { "visible" }
            else { "hidden" }>
        <div class="footer_item"
            on:click=move |_| { set_currpage(4); }>
            "Support"
        </div>
        <div class="footer_item"
            on:click=move |_| { set_currpage(5); }>
            "Careers"
        </div>
        <div class="footer_item"
            on:click=move |_| { set_currpage(6); }>
            "Privacy"
        </div>
        <div class="footer_item"
            on:click=move |_| { set_currpage(7); }>
            "Terms"
        </div>
        <div class="footer_item_noclick">
            Copyright (c) Notice
        </div>
    </div>
    
}}