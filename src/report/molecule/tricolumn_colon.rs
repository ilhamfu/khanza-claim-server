use maud::{html, Markup, Render};

pub trait OtherRenderable {
    fn tricolumn_render(&self) -> Markup;
}

macro_rules! impl_tricolumn {
    { $( $type:ty ),* $(,)? } => {
        $(
            impl OtherRenderable for $type {
                fn tricolumn_render(&self) -> Markup {
                    self.render()
                }
            }
        )*
    };
}

impl_tricolumn! {
    char,f32,f64,i8,i16,i32,i64,i128,isize,str,u8,u16,u32,u64,u128,usize,String
}

impl<T: OtherRenderable + ?Sized> OtherRenderable for &T {
    fn tricolumn_render(&self) -> Markup {
        T::tricolumn_render(self)
    }
}

impl<T: OtherRenderable> OtherRenderable for Option<T> {
    fn tricolumn_render(&self) -> Markup {
        self.as_ref().map(T::tricolumn_render).unwrap_or_default()
    }
}

impl<T: OtherRenderable> OtherRenderable for [T] {
    fn tricolumn_render(&self) -> Markup {
        html! {
            @for i in self {
                div{(i.tricolumn_render())}
            }
        }
    }
}

pub fn tricolumn_colon<T, A>(l: T, v: A) -> impl Render
where
    T: OtherRenderable,
    A: OtherRenderable,
{
    html! {
        .tricolumn {
            div {(l.tricolumn_render())}
            div {":"}
            div {(v.tricolumn_render())}
        }
    }
}
pub fn block_display<T, A>(l: T, v: A) -> impl Render
where
    T: OtherRenderable,
    A: OtherRenderable,
{
    html! {
        .block-display {
            div {(l.tricolumn_render())}
            div {(v.tricolumn_render())}
        }
    }
}
