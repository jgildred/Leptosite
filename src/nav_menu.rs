use leptos::*;

#[component]
pub fn Section(
    currpage: WriteSignal<i32>
) -> impl IntoView {
    view! { 
    // customize the nav menu layout below with divs and style classes.
    // modify or define new style classes in styles.css.
    
    <div class="top_bar">
      <div class="logo"
          on:click=move |_| {
            currpage.set(0);
          }></div>
      <div class="nav_menu">
          <div class="menu_item"
              on:click=move |_| { currpage.set(0); }>
              "Home"
          </div>
          <div class="menu_item"
              on:click=move |_| { currpage.set(1); }>
              "Products"
          </div>
          <div class="menu_item"
              on:click=move |_| { currpage.set(2); }>
              "Services"
          </div>
          <div class="menu_item"
              on:click=move |_| { currpage.set(3); }>
              "About"
          </div>
      </div>
    </div>

}}