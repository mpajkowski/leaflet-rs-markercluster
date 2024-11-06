use std::time::Duration;

use leaflet::{LatLng, Map, MapOptions, Marker, MarkerOptions, Popup, PopupOptions, TileLayer};
use leaflet_markercluster::{MarkerClusterGroup, MarkerClusterGroupOptions};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, js_sys::Array};

mod points;

#[wasm_bindgen(start)]
fn main() {
    console::log_1(&"Hello!".into());

    let options = MapOptions::default();
    let map = Map::new("map", &options);
    map.set_view(&LatLng::new(-37.79, 175.27), 5.0);
    let tile_layer = TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png");
    tile_layer.add_to(&map);

    let marker_options = MarkerClusterGroupOptions::new();
    marker_options.set_spiderfy_on_max_zoom(true);
    marker_options.set_max_cluster_radius(80.0);
    let markers = MarkerClusterGroup::new_with_options(&marker_options);

    let marker_list: Array = Array::new();

    for point in points::POINTS.iter() {
        let title = point.2;
        let marker_options = MarkerOptions::new();
        marker_options.set_title(title.to_string());
        let marker = Marker::new_with_options(&LatLng::new(point.0, point.1), &marker_options);
        let popup_options = PopupOptions::new();
        popup_options.set_pane(title.to_string());
        let popup = Popup::new(&popup_options, None);
        marker.bind_popup(&popup);
        marker_list.push(&JsValue::from(marker));
    }

    markers.add_layers(&marker_list);
    markers.add_to(&map);

    let window = web_sys::window().unwrap();

    let cb = move || {};

    let mut wrapped_cb: Option<Box<dyn FnOnce()>> = Some(Box::new(cb));
    let closure = Closure::new(move || {
        markers.remove_layers(&marker_list);
    });

    let closure = closure.into_js_value();

    let duration = Duration::from_secs(10);
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        duration.as_millis().try_into().unwrap_throw(),
    );
}
