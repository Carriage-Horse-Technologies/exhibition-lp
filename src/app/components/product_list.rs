use std::collections::HashMap;

use web_sys::{CssStyleDeclaration, HtmlElement};
use yew::prelude::*;
use yew_hooks::{use_interval, use_map, UseMapHandle};
use yewdux::prelude::use_store;

use crate::{
    app::{
        components::{modals::Modal, product::Product},
        models::{PageOffsetDomRect, ProductInfo},
        states::{CollisionState, ModalState},
    },
    my_utils::check_collision_with_page_offset_dom_rect,
    settings::GRADIENT_COLOR,
    static_data::{NEW_PRODUCT_INFO, PAST_PRODUCT_INFO_LIST},
};

#[derive(PartialEq, Properties)]
pub(crate) struct ProductListProps {
    pub(crate) myself_rect: Option<PageOffsetDomRect>,
}

#[function_component]
pub(crate) fn ProductList(props: &ProductListProps) -> Html {
    let ProductListProps { myself_rect } = props;

    let products_rect_map: UseMapHandle<String, PageOffsetDomRect> = use_map(HashMap::new());
    let (_collision_state, collision_state_dispatch) = use_store::<CollisionState>();
    let (modal_state, modal_state_dispatch) = use_store::<ModalState>();
    let gradient_counter = use_state(|| 0);
    let new_product_node = use_node_ref();

    {
        let myself_rect = myself_rect.clone();
        let products_rect_map = products_rect_map.clone();
        let collision_state_dispatch = collision_state_dispatch;
        use_effect_with_deps(
            move |(myself_rect, products_rect_map)| {
                if let Some(myself_rect) = myself_rect {
                    let mut on_collision_stay = CollisionState::default();
                    for (title, prod_rect) in products_rect_map.current().iter() {
                        log::debug!(
                            "rect-my {} {} {} {}",
                            myself_rect.top(),
                            myself_rect.bottom(),
                            myself_rect.left(),
                            myself_rect.right()
                        );
                        log::debug!(
                            "rect-prod {} {} {} {} {}",
                            title,
                            prod_rect.top(),
                            prod_rect.bottom(),
                            prod_rect.left(),
                            prod_rect.right()
                        );
                        if check_collision_with_page_offset_dom_rect(myself_rect, prod_rect) {
                            let mut all_product_info_list = PAST_PRODUCT_INFO_LIST.to_vec();
                            all_product_info_list.push(NEW_PRODUCT_INFO);
                            on_collision_stay = CollisionState {
                                on_collision_stay: true,
                                url: all_product_info_list
                                    .iter()
                                    .find(|prod_info| prod_info.title == title)
                                    .unwrap()
                                    .url
                                    .to_string(),
                            };
                            log::debug!("hit! {}", title);
                        }
                    }
                    // 何も接触していなかったらfalse(default)でreduce．
                    collision_state_dispatch.reduce(|_| on_collision_stay.into());
                }
            },
            (myself_rect, products_rect_map),
        );
    }

    {
        let gradient_counter = gradient_counter.clone();
        let new_product_node = new_product_node.clone();
        use_interval(
            move || {
                let new_product_element = new_product_node.cast::<HtmlElement>().unwrap();
                let style = new_product_element.style();

                set_gradient(&GRADIENT_COLOR, &style, *gradient_counter);
                gradient_counter.set((*gradient_counter + 1) % GRADIENT_COLOR.len())
            },
            100,
        );
    }

    html! {
        <div>
            <Product node_ref={new_product_node} classes={classes!("container", "mx-auto", "mt-[300px]", "mb-[100px]",
                            "bg-gradient-to-r", "from-[#6080B0]", "via-[#08DCF9]", "to-[#FF2775]", "p-2"
                            )} product_info={NEW_PRODUCT_INFO} rect_map={products_rect_map.clone()} new={true} />
            <div class={classes!("grid", "grid-cols-3", "justify-items-center", "place-content-around", "place-items-center")}>
                {
                    for PAST_PRODUCT_INFO_LIST.iter().map(|info| {
                        html! {
                            <Product product_info={(*info).clone()} rect_map={products_rect_map.clone()} />
                        }
                    })
                }
                if modal_state.is_display {
                    <Modal />
                }
            </div>
        </div>
    }
}

fn set_gradient(gradient_color: &Vec<&str>, style: &CssStyleDeclaration, current_index: usize) {
    let grad_num = gradient_color.len();
    let mut property_value = "linear-gradient(to top left, ".to_string();
    for i in current_index..(current_index + grad_num) {
        property_value.push_str(&gradient_color[i % grad_num]);
        property_value.push_str(",");
    }
    let offset = property_value.rfind(",").unwrap();
    property_value.replace_range(offset.., "");
    property_value.push_str(")");
    style.set_property("background", &property_value).unwrap();
}
