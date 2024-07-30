// in most cases this files does not need to be modified

use leptos::*;
use crate::nav_menu;
use crate::pages;

#[component]
pub fn Section() -> impl IntoView {
    let (currpage, set_currpage) = create_signal(0);
    set_currpage(0); // this is the page that will show by default
    view! {
    // this should include all pages you want to be available on the web site
    
    <nav_menu::Section currpage=set_currpage/>
    <pages::home::Section currpage=currpage set_currpage=set_currpage/>
    <pages::products::Section currpage=currpage set_currpage=set_currpage/>
    <pages::services::Section currpage=currpage set_currpage=set_currpage/>
    <pages::about::Section currpage=currpage set_currpage=set_currpage/>

}}