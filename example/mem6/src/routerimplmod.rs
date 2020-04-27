//! routerimplmod - A simple `#`-fragment router for dodrio html templating.  
//! Implementation of Router for this mem6 use case with RootRenderingComponent type
//! It routes from short_url (the url hash part) to a
//! html_template file to fetch. The file name is written to rrc.local_route.  
//! Then fetches the file and stores it in rrc.html_template

use crate::*;
use dodrio::VdomWeak;
//use unwrap::unwrap;
use rust_wasm_router::routermod::{RouterTrait};

/// The struct must be declared near the implementation, not definition of the Trait
pub struct Router {
    /// local # hash route
    pub local_route: String,
}

impl Router {
    /// constructor
    pub fn new() -> Self {
        // return from constructor
        Self {
            local_route: "".to_string(),
        }
    }
}

impl RouterTrait for Router {
    /// access methods to underlying fields
    fn get_local_route_from_self(&self) -> &str {
        //return
        &self.local_route
    }
    /// get rrc.local_route
    fn get_local_route_from_root(root: &mut dyn dodrio::RootRender) -> &str {
        let rrc = root.unwrap_mut::<RootRenderingComponent>();
        &rrc.router_data.local_route
    }

    /// update local_route with filenames dependent on short_local_route.
    fn update_local_route_from_root(
        short_local_route: String,
        root: &mut dyn dodrio::RootRender,
        vdom: VdomWeak,
    ) -> String {
        let rrc = root.unwrap_mut::<RootRenderingComponent>();
        // there are 2 entry points: no hash and #p03
        if short_local_route == "#p02" {
            fetchmod::async_fetch_game_config_and_update(rrc, vdom);
            rrc.router_data.local_route = "p02_start_a_group.html".to_owned();
        } else if short_local_route.starts_with("#p03") {
            // entry point for join game
            rrc.start_websocket(vdom.clone());
            rrc.game_data.my_player_number = 2;
            if short_local_route.contains('.') {
                let gr = Self::get_url_param_in_hash_after_dot(&short_local_route);
                storagemod::save_group_id_string_to_local_storage(rrc, gr);
            } else {
                storagemod::load_group_id_string(rrc);
            }
            rrc.router_data.local_route = "p03_join_a_group.html".to_owned();
        } else if short_local_route == "#p04" {
            statusjoinedmod::on_load_joined(rrc);
            rrc.router_data.local_route = "p04_wait_to_start.html".to_owned();
        } else if short_local_route == "#p05" {
            rrc.router_data.local_route = "p05_choose_game.html".to_owned();
        } else if short_local_route == "#p06" {
            rrc.router_data.local_route = "p06_drink.html".to_owned();
        } else if short_local_route == "#p07" {
            rrc.router_data.local_route = "p07_do_not_drink.html".to_owned();
        } else if short_local_route == "#p08" {
            rrc.router_data.local_route = "p08_instructions.html".to_owned();
        } else if short_local_route == "#p11" {
            rrc.router_data.local_route = "p11_gameboard.html".to_owned();
        } else if short_local_route == "#p21" {
            rrc.router_data.local_route = "p21_menu.html".to_owned();
        } else if short_local_route == "#p31" {
            rrc.router_data.local_route = "p31_debug_text.html".to_owned();
        } else if short_local_route == "#p41" {
            // entry point for webrtc chat
            rrc.start_websocket(vdom.clone());
            rrc.router_data.local_route = "p41_webrtc.html".to_owned();
        } else {
            // main entry point
            rrc.router_data.local_route = "p01_start.html".to_owned();
        }
        // return
        rrc.router_data.local_route.to_string()
    }

    /// update html_template and extract and saves html_sub_templates
    #[allow(clippy::integer_arithmetic)]
    #[allow(clippy::indexing_slicing)]
    fn update_html_template_and_sub_templates(
        resp_body_text: String,
    ) -> Box<dyn Fn(&mut dyn dodrio::RootRender) + 'static> {
        // Callback fired whenever the URL hash fragment changes.
        // Keeps the rrc.router_data.local_route in sync with the `#` fragment.
        Box::new(move |root| {
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            htmltemplateimplmod::update_html_template_and_sub_templates(rrc, &resp_body_text);
        })
    }
}
