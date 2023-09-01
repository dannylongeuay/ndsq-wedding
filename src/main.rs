use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (selected, set_selected) = create_signal(cx, SlotSelection::Landing);
    view! { cx,
        <main class="container">
        <Header selection=selected set_selection=set_selected/>
        <Slot selection=selected>
            <Landing slot >
                <LandingSlot set_selection=set_selected/>
            </Landing>
            <Photos slot>
                <PhotosSlot/>
            </Photos>
            <Photobooth slot>
                <PhotoboothSlot/>
            </Photobooth>
            <Clips slot>
                <ClipsSlot/>
            </Clips>
        </Slot>
        </main>
    }
}

#[component]
fn Header(
    cx: Scope,
    selection: ReadSignal<SlotSelection>,
    set_selection: WriteSignal<SlotSelection>,
) -> impl IntoView {
    view! { cx,
        { move || if selection.get() != SlotSelection::Landing {
                view! { cx,
                    <button
                    on:click=move |_| { set_selection.set(SlotSelection::Landing)}
                    >Back</button>
                }.into_any()
            } else {
                view! { cx,
                    <h1>"ND Wedding Gallery"</h1>
                }.into_any()
        }}
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
                document().body().expect("body to be present").scroll_into_view();
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
                document().body().expect("body to be present").scroll_into_view();
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
                document().body().expect("body to be present").scroll_into_view();
                set_selection.set(SlotSelection::Clips);
            }
            >View</button>
        </article>
    }
}

#[component]
fn InfiniteScroller(
    cx: Scope,
    #[prop(default = "https://ik.imagekit.io/dannylongeuay/ndsq/nd_fave_{}.jpg?tr=h-800")]
    url_format: &'static str,
    #[prop(default = "nd_fave_{}")] alt_format: &'static str,
    #[prop(default = 32)] photos_length: i32,
) -> impl IntoView {
    let photos_start = 1;
    let photos_end = 5;
    let initial_indexes: Vec<i32> = (photos_start..=photos_end).collect();
    let (photo_ids, set_photo_ids) = create_signal(cx, initial_indexes);
    let add_photos = move |_| {
        let scroller = document().get_element_by_id("scroller").expect("exists");
        let last_el = scroller.last_element_child().unwrap();
        let last_el_id = last_el.get_attribute("id").unwrap();
        let last_el_top = last_el.get_bounding_client_rect().top();
        set_photo_ids.update(|photo_ids| {
            if let Some(last) = photo_ids.last().cloned() {
                for i in 0..4 {
                    photo_ids.push(((last + 1 + i) % photos_length) + 1);
                }
            }
            while photo_ids.len() > 10 {
                photo_ids.remove(0);
            }
        });
        let shifted_el = document().get_element_by_id(&last_el_id).unwrap();
        let shifted_el_top = shifted_el.get_bounding_client_rect().top();
        let el_diff = shifted_el_top - last_el_top;
        let new_scroll_y = window().scroll_y().unwrap() + el_diff;
        window().scroll_to_with_x_and_y(0.0, new_scroll_y);
    };
    view! { cx,
        <div id="scroller">
        <For
            each=move || photo_ids.get()
            key=|&photo_id| photo_id
            view=move |cx, id|
            view! { cx,
                <article id=move || alt_format.replace("{}", &id.to_string())>
                    <img
                        src=move || url_format.replace("{}", &id.to_string())
                        alt=move || alt_format.replace("{}", &id.to_string())
                    />
                </article>
            }
        />
        </div>
        <button
            on:click=add_photos
        >Load More</button>
    }
}

#[slot]
struct Photos {
    children: ChildrenFn,
}

#[component]
fn PhotosSlot(cx: Scope) -> impl IntoView {
    view! { cx,
        <InfiniteScroller/>
    }
}

#[slot]
struct Photobooth {
    children: ChildrenFn,
}

#[component]
fn PhotoboothSlot(cx: Scope) -> impl IntoView {
    view! { cx,
        <InfiniteScroller
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
fn ClipsSlot(cx: Scope) -> impl IntoView {
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
    let clips_start = 0 as usize;
    let clips_end = 4 as usize;
    let initial_indexes: Vec<usize> = (clips_start..=clips_end).collect();
    let (clip_ids, set_clip_ids) = create_signal(cx, initial_indexes);
    let (clip_names, set_clip_names) = create_signal(cx, clips[clips_start..=clips_end].to_vec());
    let add_clips = move |_| {
        let scroller = document().get_element_by_id("scroller").expect("exists");
        let last_el = scroller.last_element_child().unwrap();
        let last_el_id = last_el.get_attribute("id").unwrap();
        let last_el_top = last_el.get_bounding_client_rect().top();
        set_clip_ids.update(|clip_ids| {
            if let Some(last) = clip_ids.last().cloned() {
                for i in 0..4 {
                    clip_ids.push((last + i + 1) % clips.len());
                }
            }
            while clip_ids.len() > 9 {
                clip_ids.remove(0);
            }
        });
        let mut next_clips: Vec<&str> = Vec::new();
        for clip_id in clip_ids.get() {
            next_clips.push(clips[clip_id])
        }
        set_clip_names.set(next_clips);
        let shifted_el = document().get_element_by_id(&last_el_id).unwrap();
        let shifted_el_top = shifted_el.get_bounding_client_rect().top();
        let el_diff = shifted_el_top - last_el_top;
        let new_scroll_y = window().scroll_y().unwrap() + el_diff;
        window().scroll_to_with_x_and_y(0.0, new_scroll_y);
    };
    view! { cx,
        <div id="scroller">
        <For
            each=move || clip_names.get()
            key=|&clip_name| clip_name
            view=move |cx, clip_name|
            view! { cx,
                <article id=move|| format!("{}", clip_name)>
                    <video autoplay loop muted playsinline width="300">
                        <source src=move || {
                            format!("https://ik.imagekit.io/dannylongeuay/ndsq/{}.mp4", clip_name)
                        } type="video/mp4"/>
                    </video>
                </article>
            }
        />
        </div>
        <button
            on:click=add_clips
        >Load More</button>
    }
}
