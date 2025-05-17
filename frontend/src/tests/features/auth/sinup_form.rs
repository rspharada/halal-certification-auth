use halal_certification_auth::features::auth::components::signup_form::SignupForm;
use wasm_bindgen_test::*;
use yew::Renderer;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_renders_signup_form() {
    let output = gloo::utils::document().get_element_by_id("output").unwrap();
    Renderer::<SignupForm>::with_root(output).render();
}
