use yew::prelude::*;

#[function_component(LockIcon)]
pub fn lock_icon() -> Html {
    // ph:lock-simple-fill
    html!(
        <svg width="16" height="16" viewBox="0 0 256 256">
            <path fill="currentColor" d="M208 80h-36V52a44 44 0 0 0-88 0v28H48a16 16 0 0 0-16 16v112a16 16 0 0 0 16 16h160a16 16 0 0 0 16-16V96a16 16 0 0 0-16-16ZM100 52a28 28 0 0 1 56 0v28h-56Z" />
        </svg>
    )
}

#[function_component(HeartIcon)]
pub fn heart_icon() -> Html {
    // twemoji:red-heart
    html!(
        <svg width="16" height="16" viewBox="0 0 36 36">
            <path fill="#DD2E44" d="M35.885 11.833c0-5.45-4.418-9.868-9.867-9.868c-3.308 0-6.227 1.633-8.018 4.129c-1.791-2.496-4.71-4.129-8.017-4.129c-5.45 0-9.868 4.417-9.868 9.868c0 .772.098 1.52.266 2.241C1.751 22.587 11.216 31.568 18 34.034c6.783-2.466 16.249-11.447 17.617-19.959c.17-.721.268-1.469.268-2.242z" />
        </svg>
    )
}
