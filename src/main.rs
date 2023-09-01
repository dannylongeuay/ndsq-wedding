use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (selected, set_selected) = create_signal(cx, SlotSelection::Landing);
    view! { cx,
        <div id="scroll" class="scroll">
        <main class="container">
        <Slot selection=selected>
            <Landing slot >
                <LandingSlot set_selection=set_selected/>
            </Landing>
            <Photos slot>
                <PhotosSlot set_selection=set_selected/>
            </Photos>
            <Photobooth slot>
                <PhotoboothSlot set_selection=set_selected/>
            </Photobooth>
            <Clips slot>
                <ClipsSlot set_selection=set_selected/>
            </Clips>
        </Slot>
        </main>
        </div>
    }
}

#[component]
fn Header(
    cx: Scope,
    set_selection: WriteSignal<SlotSelection>,
    interval_handle: IntervalHandle,
) -> impl IntoView {
    view! { cx,
        <nav class="container-fluid">
            <ul>
                <li>
                    <button
                    id="back"
                    on:click=move |_| {
                        set_selection.set(SlotSelection::Landing);
                        interval_handle.clear();
                    }
                    >Back</button>
                </li>
            </ul>
        </nav>
    }
}

#[derive(Clone, PartialEq)]
enum SlotSelection {
    Landing,
    Photos,
    Photobooth,
    Clips,
}

#[component]
fn Slot(
    cx: Scope,
    selection: ReadSignal<SlotSelection>,
    landing: Landing,
    photos: Photos,
    photobooth: Photobooth,
    clips: Clips,
) -> impl IntoView {
    move || match selection.get() {
        SlotSelection::Photos => (photos.children)(cx).into_view(cx),
        SlotSelection::Photobooth => (photobooth.children)(cx).into_view(cx),
        SlotSelection::Clips => (clips.children)(cx).into_view(cx),
        SlotSelection::Landing => (landing.children)(cx).into_view(cx),
    }
}

#[slot]
struct Landing {
    children: ChildrenFn,
}

#[component]
fn LandingSlot(cx: Scope, set_selection: WriteSignal<SlotSelection>) -> impl IntoView {
    view! { cx,
        <nav class="container-fluid">
            <ul/>
            <ul>
                <li>
                    <h1>"ND Wedding Gallery"</h1>
                </li>
            </ul>
            <ul/>
        </nav>
        <article>
            <header>Photos</header>
            <section>
                <img
                    src="https://ik.imagekit.io/dannylongeuay/ndsq/nd_fave_4.jpg?tr=h-600"
                    alt="nd_fave_hero"
                />
            </section>
            <button
            on:click=move |_| {
                document()
                    .get_element_by_id("scroll")
                    .expect("scroll div to be present")
                    .scroll_to_with_x_and_y(0.0, 0.0);
                set_selection.set(SlotSelection::Photos);
            }
            >View</button>
        </article>
        <article>
            <header>Photobooth</header>
            <section>
                <img
                    src="https://ik.imagekit.io/dannylongeuay/ndsq/nd_photobooth_1.jpg?tr=h-600"
                    alt="nd_photobooth_hero"
                />
            </section>
            <button
            on:click=move |_| {
                document()
                    .get_element_by_id("scroll")
                    .expect("scroll div to be present")
                    .scroll_to_with_x_and_y(0.0, 0.0);
                set_selection.set(SlotSelection::Photobooth);
            }
            >View</button>
        </article>
        <article>
            <header>Clips</header>
            <section>
                <video autoplay loop muted playsinline width="300">
                    <source src="https://ik.imagekit.io/dannylongeuay/ndsq/nd_pond_pan.mp4" type="video/mp4" />
                </video>
            </section>
            <button
            on:click=move |_| { set_selection.set(SlotSelection::Clips)}
            on:click=move |_| {
                document()
                    .get_element_by_id("scroll")
                    .expect("scroll div to be present")
                    .scroll_to_with_x_and_y(0.0, 0.0);
                set_selection.set(SlotSelection::Clips);
            }
            >View</button>
        </article>
    }
}

#[component]
fn InfiniteScroller(
    cx: Scope,
    set_selection: WriteSignal<SlotSelection>,
    #[prop(default = "https://ik.imagekit.io/dannylongeuay/ndsq/nd_fave_{}.jpg?tr=h-800")]
    url_format: &'static str,
    #[prop(default = "nd_fave_{}")] alt_format: &'static str,
    #[prop(default = 32)] photos_length: i32,
) -> impl IntoView {
    let photos_start = 1;
    let photos_end = 5;
    let initial_indexes: Vec<i32> = (photos_start..=photos_end).collect();
    let (photo_ids, set_photo_ids) = create_signal(cx, initial_indexes);
    let add_photos = move || {
        if let Some(scroll_div) = document().get_element_by_id("scroll") {
            let scroll_top = scroll_div.scroll_top();
            let client_height = scroll_div.client_height();
            let scroll_height = scroll_div.scroll_height();
            if scroll_top + client_height >= scroll_height - 400 {
                set_photo_ids.update(|photo_ids| {
                    photo_ids.remove(0);
                    if let Some(last) = photo_ids.last() {
                        photo_ids.push((last % photos_length) + 1);
                    }
                });
            }
        }
    };
    let handle_res = set_interval_with_handle(add_photos, std::time::Duration::from_millis(1000));
    let interval_handle = handle_res.unwrap();
    view! { cx,
        <Header set_selection interval_handle/>
        <For
            each=move || photo_ids.get()
            key=|&photo_id| photo_id
            view=move |cx, id|
            view! { cx,
                <article>
                    <img
                        src=move || url_format.replace("{}", &id.to_string())
                        alt=move || alt_format.replace("{}", &id.to_string())
                    />
                </article>
            }
        />
    }
}

#[slot]
struct Photos {
    children: ChildrenFn,
}

#[component]
fn PhotosSlot(cx: Scope, set_selection: WriteSignal<SlotSelection>) -> impl IntoView {
    view! { cx,
        <InfiniteScroller set_selection=set_selection/>
    }
}

#[slot]
struct Photobooth {
    children: ChildrenFn,
}

#[component]
fn PhotoboothSlot(cx: Scope, set_selection: WriteSignal<SlotSelection>) -> impl IntoView {
    view! { cx,
        <InfiniteScroller
            set_selection
            url_format="https://ik.imagekit.io/dannylongeuay/ndsq/nd_photobooth_{}.jpg?tr=h-800"
            alt_format="nd_photobooth_{}"
            photos_length=58
        />
    }
}

#[slot]
struct Clips {
    children: ChildrenFn,
}

#[component]
fn ClipsSlot(cx: Scope, set_selection: WriteSignal<SlotSelection>) -> impl IntoView {
    let clips = vec![
        "nd_down_the_aisle",
        "nd_ceremony_armpull",
        "nd_up_the_aisle",
        "nd_cheer",
        "nd_pond_pan",
        "nd_jump",
        "nd_dance_twirl",
        "nd_father_daughter_dance",
        "nd_silly_butt",
        "nd_trio_trouble",
        "nd_brother_groove",
        "nd_getting_down_n_dirty",
        "nd_family_pan",
        "nd_final_send_off",
        "nd_final_kiss",
    ];
    let clips_start = 0;
    let clips_end = 4;
    let initial_indexes: Vec<usize> = (clips_start..=clips_end).collect();
    let (clip_ids, set_clip_ids) = create_signal(cx, initial_indexes);
    let (clip_names, set_clip_names) = create_signal(cx, clips[clips_start..=clips_end].to_vec());
    let add_clips = move || {
        if let Some(scroll_div) = document().get_element_by_id("scroll") {
            let scroll_top = scroll_div.scroll_top();
            let client_height = scroll_div.client_height();
            let scroll_height = scroll_div.scroll_height();
            if scroll_top + client_height >= scroll_height - 200 {
                set_clip_ids.update(|clip_ids| {
                    clip_ids.remove(0);
                    if let Some(last) = clip_ids.last() {
                        clip_ids.push((last + 1) % clips.len());
                    }
                });
                let mut next_clips: Vec<&str> = Vec::new();
                for clip_id in clip_ids.get() {
                    next_clips.push(clips[clip_id])
                }
                set_clip_names.set(next_clips);
            }
        }
    };
    let handle_res = set_interval_with_handle(add_clips, std::time::Duration::from_millis(1000));
    let interval_handle = handle_res.unwrap();
    view! { cx,
        <Header set_selection interval_handle/>
        <For
            each=move || clip_names.get()
            key=|&clip_name| clip_name
            view=move |cx, clip_name|
            view! { cx,
                <article>
                    <video autoplay loop muted playsinline width="300">
                        <source src=move || {
                            format!("https://ik.imagekit.io/dannylongeuay/ndsq/{}.mp4", clip_name)
                        } type="video/mp4"/>
                    </video>
                </article>
            }
        />
    }
}
