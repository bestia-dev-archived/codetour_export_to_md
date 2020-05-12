# codetour_start_route_template_render
The first tour describes the start of the wasm module, the routing, the html templating and finally rendering.
### index.html wasm import script
The browser downloads the index.html file as usual. 
This \<script\> element imports and init the wasm code.  
A simple web file server is enough. 
Warning: the index.html cannot be served from local file without a server, because importing modules is not allowed then for security reasons.
I will try to use only Rust+Wasm and avoid JavaScript as much as possible.

##### step 1 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/webfolder/mem6/index.html#L82)
```html
                  the game...<br>
                  This is <br>
                  very quick on fast<br>
                  networks...<br>
            </h2>
      </div>
#//---------------------- selection start ----------------------
      <!-- import and init the wasm code -->
      <script type="module">
            import init from "./pkg/mem6.js";
            init("./pkg/mem6_bg.wasm");
      </script>
#//----------------------- selection end -----------------------
```
### wasm_bindgen_start
Rust is a great language to compile to Wasm/WebAssembly.
With the use of libraries wasm_bindgen, web_sys and js_sys Rust has access to the browser javascript environment. So it can do most of the things that JavaScript can do.  
The imported Wasm module will automatically start the function with attribute `#[wasm_bindgen(start)]`. This function is called only once. 

##### step 2 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/lib.rs#L299)
```rust
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::gamedatamod::*;

use rust_wasm_dodrio_templating::*;
use rust_wasm_websys_utils::*;
//use rust_wasm_websocket::*;

// use unwrap::unwrap;
use wasm_bindgen::prelude::*;

#//---------------------- selection start ----------------------
#[wasm_bindgen(start)]
#//----------------------- selection end -----------------------
```
### div_for_virtual_dom
The Rust code will change just the content of the \<div id="div_for_virtual_dom"\>. 
This is a "single page web app". For the browser the index.html is always the same, we only change the interior content of it's dom. 

##### step 3 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/lib.rs#L306)
```rust
#[wasm_bindgen(start)]
#[allow(clippy::shadow_same)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    websysmod::debug_write(&format!("wasm app version: {}", env!("CARGO_PKG_VERSION")));

    // Get the container to render the virtual Dom component.
#//---------------------- selection start ----------------------
    let div_for_virtual_dom = websysmod::get_element_by_id("div_for_virtual_dom");
#//----------------------- selection end -----------------------
```
### div_for_virtual_dom in index.html
This div is the only part of the index.html that will be dynamically changed by the Rust code.  
The download of a big wasm file can take some time on slow network. It is nice to warn the user about that.  

##### step 4 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/webfolder/mem6/index.html#L77)
```html
#//---------------------- selection start ----------------------
      <!-- display a text while waiting for wasm download. It can take some time. -->
      <div id="div_for_virtual_dom">
            <h2>
                  Waiting to<br>
                  download <br>
                  the game...<br>
                  This is <br>
                  very quick on fast<br>
                  networks...<br>
            </h2>
      </div>
#//----------------------- selection end -----------------------
```
### noscript warning
Wasm is using the javascript engine deep inside.  
If JavaScript is disabled, also wasm cannot run.  

##### step 5 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/webfolder/mem6/index.html#L66)
```html
      <script src="start_service_worker.js"></script>
#//---------------------- selection start ----------------------
      <!-- warning if javascript iis not enabled -->
      <noscript>
            <h2>
                  !!!???!!!<br>
                  This game <br>
                  cannot work <br>
                  without javascript<br>
                  enabled<br>
                  !!!???!!!</h2>
      </noscript>
#//----------------------- selection end -----------------------
```
 ### start_router
 After preparing the environment (websocket, RootRenderingComponent, vdom, fetch config data) I start the router. It will listen to the event `hashchange`.  
 For example when the URL changes to index.html#p04.

##### step 6 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/lib.rs#L325)
```rust
    // Mount the component to the `<div id="div_for_virtual_dom">`.
    let vdom_object = dodrio::Vdom::new(&div_for_virtual_dom, rrc);
    let vdom = vdom_object.weak();
    // async fetch_response() for gamesmetadata.json
    fetchmod::fetch_games_metadata_and_update(&location_href, vdom.clone());
    fetchmod::fetch_videos_and_update(&location_href, vdom.clone());
    fetchmod::fetch_audio_and_update(&location_href, vdom.clone());
    // Start the URL router.
    use rust_wasm_router::routermod::RouterTrait;
    let router = routerimplmod::Router::new();
#//---------------------- selection start ----------------------
    router.start_router(vdom.clone());
#//----------------------- selection end -----------------------
```
### update_local_route_from_root
The short_local_route (url hash) `ex. #p04` defines a `local_route`. This is the name of the html template to fetch from the web server. Than it is prepared and saved in html_template sub_templates fields.
The data in the struct is prepared, finally we call `vdom.schedule_render();`.

##### step 7 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/routerimplmod.rs#L45)
```rust
        //return
        &self.local_route
    }
    /// get rrc.local_route
    fn get_local_route_from_root(root: &mut dyn dodrio::RootRender) -> &str {
        let rrc = root.unwrap_mut::<RootRenderingComponent>();
        &rrc.router_data.local_route
    }

    /// update local_route with filenames dependent on short_local_route.
#//---------------------- selection start ----------------------
    fn update_local_route_from_root(
#//----------------------- selection end -----------------------
```
### render()
This is the only method that is called when the rendering is scheduled. 
It is defined in the crate `dodrio: the vdom library`.   
From here we then call functions to render different UI depending on the data state. 

##### step 8 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/rootrenderingcomponentmod.rs#L51)
```rust
    pub fn start_websocket(&mut self, vdom: VdomWeak) {
        self.web_data.start_websocket(vdom);
        self.web_data.web_rtc_data.rtc_ws = self.web_data.websocket_data.ws.clone();
    }
}

///`Render` trait implementation on RootRenderingComponent struct
/// It is called for every Dodrio animation frame to render the vdom.
/// Only when render is scheduled after some change id the game data.
impl<'a> Render<'a> for RootRenderingComponent {
#//---------------------- selection start ----------------------
    fn render(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
#//----------------------- selection end -----------------------
```
### dodrio
\<github.com/fitzgen/dodrio\> is a virtual DOM library for Rust+Wasm.  
The vdom approach aims to make it easier for the developer to think about the dom changes. The classic approach is to react to en event and then programatically modify the dom. The problem is that it becomes quickly very complex. It is difficult to think (and test) an outcome after a long sequence of events.  
It is easier to think about the dom as a "state machine". The virtual dom is rendered from scratch every single time. The rendering depends only on the `state data` that is now in the data model (a struct called RootRenderingComponent). So changing this struct (data model) and scheduling the rendering, will result in a different vdom. No other complications around that. Pure simple. Easy to think, easy to test.     
Then the original dom is modified to resemble the vdom. Modifying the dom is slow. To make it efficient, there is some smart caching and diff-ing in the library, to make the number of changes as small and efficient as possible.  
In this way we have a clear separation between data (RootRenderingComponent) and UI (code to render html virtual dom).  

##### step 9 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/Cargo.toml#L29)
```toml
# region: my dependencies
rust_wasm_websys_utils = "0.4.2"
# rust_wasm_websys_utils = { path = "../../rust_wasm_websys_utils" }
rust_wasm_router = { path = "../../rust_wasm_router" }
rust_wasm_dodrio_templating = { path = "../../rust_wasm_dodrio_templating" }
rust_wasm_websocket = { path = "../../rust_wasm_websocket" }
rust_wasm_webrtc = { path = "../../rust_wasm_webrtc" }
#qrcode53bytes = { path = "../../qrcode53bytes" }
qrcode53bytes ="1.0.0"
# endregion: my dependencies

```
### crates.io or github or path
If the newest version of dodrio on crates.io is not actual enought,  
in cargo.toml we can depend on github or local path also.  
So we can follow the codeflow and change something if we need to.  

##### step 10 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/Cargo.toml#L31)
```toml
# rust_wasm_websys_utils = { path = "../../rust_wasm_websys_utils" }
rust_wasm_router = { path = "../../rust_wasm_router" }
rust_wasm_dodrio_templating = { path = "../../rust_wasm_dodrio_templating" }
rust_wasm_websocket = { path = "../../rust_wasm_websocket" }
rust_wasm_webrtc = { path = "../../rust_wasm_webrtc" }
#qrcode53bytes = { path = "../../qrcode53bytes" }
qrcode53bytes ="1.0.0"
# endregion: my dependencies
#//---------------------- selection start ----------------------

# region: other dependencies
unwrap = "1.2.1"
#//----------------------- selection end -----------------------
```
### data model
The struct RootRenderingComponent contains ALL the data that is needed to render the UI. I separated the data in sub-structs, just for clarity.  
When an event occures, we just change the data and then schedule the rendering.  
Inside the event code we don't bother about the dom change !    
We have a clear separation between data and User-Interface because of that.  


##### step 11 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/rootrenderingcomponentmod.rs#L22)
```rust

#//---------------------- selection start ----------------------
/// Root Rendering Component has all
/// the data needed for play logic and rendering
pub struct RootRenderingComponent {
    /// data for web and communication
    pub web_data: webdatamod::WebData,
    /// game data will be inside of Root
    pub game_data: gamedatamod::GameData,
    /// router data
    pub router_data: routerimplmod::Router,
}
#//----------------------- selection end -----------------------
```
### render_template ()
The function `render_template` will read the html of the template and create dodrio vdom elements in the same sort order. Before some elements/attributes there are comments or `data-` attributes that change the next element/attribute. These are not visible, so the template is still a regular html document that the graphical designer can look and modify statically without dinamic elements. The programmer than adds/modify the `replace elements or attributes`.  
The result is the `dodrio:Node` that represents the vdom.  
The vdom library then diffs and modify the real dom.  

##### step 12 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/rootrenderingcomponentmod.rs#L64)
```rust
        // html fragment from html_template defined in # local_route
        if self.web_data.html_template.is_empty() {
            htmltemplatemod::empty_div(cx)
        } else {
#//---------------------- selection start ----------------------
            // i must add use crate::htmltemplatemod::HtmlTemplating;
            // to allow this trait to be used here on self
            unwrap!(self.render_template(
                cx,
                &self.web_data.html_template,
                htmltemplatemod::HtmlOrSvg::Html,
            ))
#//----------------------- selection end -----------------------
```
### templating variables
Here we can see a html template with the replace `comments` ex. \<!--t=game_name--\> and `data- attributes` ex. `data-on-click="game_type_left_onclick"`.

##### step 13 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/webfolder/mem6/p05_choose_game.html#L23)
```html
  <link rel="stylesheet" href="css/mem6.css">
</head>

<body>
  <svg height="100%" width="100%">
    <text x="50%" y="15%" class="h1 clickable orange bold"
          data-on-click="open_youtube">unForGetTable</text>

    <text x="50%" y="50%" class="h6">Choose a type of game</text>
    <rect class="rounded green clickable" x="5%" y="52.5%" width="10%"
#//---------------------- selection start ----------------------
          height="10%" data-on-click="game_type_left_onclick" />
#//----------------------- selection end -----------------------
```
### call_fn_string
This fn will replace the next text element after \<!--t=fn_name--\> or the next attribute value after `data-t-style="fn_name"` with a string.

##### step 14 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/htmltemplateimplmod.rs#L41)
```rust
            }
        }
    }

    /// html_templating functions that return a String
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
#//---------------------- selection start ----------------------
    fn call_fn_string(&self, fn_name: &str) -> String {
#//----------------------- selection end -----------------------
```
### call_fn_node
This fn will replace the next element after \<!--n=fn_name--\> with a Node.

##### step 15 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/htmltemplateimplmod.rs#L261)
```rust
                _ => {
                    let x = format!("Error: Unrecognized call_fn_listener: \"{}\"", fn_name);
                    websysmod::debug_write(&x);
                }
            }
        })
    }

    /// html_templating functions that return a Node
    #[allow(clippy::needless_return)]
#//---------------------- selection start ----------------------
    fn call_fn_node<'a>(&self, cx: &mut RenderContext<'a>, fn_name: &str) -> Node<'a> {
#//----------------------- selection end -----------------------
```
### call_fn_vec_nodes
This fn will replace the next element after \<!--vn=fn_name--\> with a Vector of Nodes.

##### step 16 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/htmltemplateimplmod.rs#L296)
```rust
                    )])
                    .finish();

                return node;
            }
        }
    }

    /// html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
#//---------------------- selection start ----------------------
    fn call_fn_vec_nodes<'a>(&self, cx: &mut RenderContext<'a>, fn_name: &str) -> Vec<Node<'a>> {
#//----------------------- selection end -----------------------
```
### call_fn_boolean
This fn will remove the next element after \<!--b=fn_name--\> if the result is `false`.

##### step 17 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/htmltemplateimplmod.rs#L19)
```rust
use dodrio::{
    Node, RenderContext, RootRender,
    bumpalo::{self},
    builder::{ElementBuilder, text},
    VdomWeak,
};
use web_sys::{Event};

impl htmltemplatemod::HtmlTemplating for RootRenderingComponent {
    /// html_templating boolean id the next node is rendered or not
#//---------------------- selection start ----------------------
    fn call_fn_boolean(&self, fn_name: &str) -> bool {
#//----------------------- selection end -----------------------
```
### call_fn_listener
This fn will add a listener to the element after `data-on-click="fn_name"`.

##### step 18 of 18 [View code in GitHub](https://github.com/LucianoBestia/mem6_game/blob/master/mem6/src/htmltemplateimplmod.rs#L98)
```rust
            _ => {
                let x = format!("Error: Unrecognized call_fn_string: \"{}\"", fn_name);
                websysmod::debug_write(&x);
                x
            }
        }
    }

    /// return a closure for the listener.
    #[allow(clippy::too_many_lines, clippy::type_complexity)]
#//---------------------- selection start ----------------------
    fn call_fn_listener(
#//----------------------- selection end -----------------------
```
