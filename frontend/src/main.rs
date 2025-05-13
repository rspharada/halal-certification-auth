mod app;
mod components;
mod pages;
mod routes;
mod types;
mod utils;

// エントリーポイント：Yewアプリケーションの起動
fn main() {
    // Appコンポーネントをマウント（HTMLの<body><main>に描画）
    yew::Renderer::<app::App>::new().render();
}
