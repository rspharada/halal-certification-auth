mod app;
mod pages;
mod routes;
mod types;
mod utils;
// mod components;

// エントリーポイント：Yewアプリケーションの起動
fn main() {
    // Appコンポーネントをマウント（HTMLの<body><main>に描画）
    yew::Renderer::<app::App>::new().render();
}
