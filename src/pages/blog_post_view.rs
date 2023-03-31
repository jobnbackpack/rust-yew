use yew::prelude::*;
use crate::blog_post;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub name: String,
}

pub struct PostComponent {
    post: blog_post::Post,
}

impl Component for PostComponent {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            post: blog_post::Post::generate_from_name(ctx.props().name.clone()),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.post = blog_post::Post::generate_from_name(ctx.props().name.clone());
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { post } = self;

        html! {
            <>
                <div>
                    <h1 class="title">
                        { &post.meta.title }
                    </h1>
                </div>
                <section>
                    { self.post.content.clone() }
                </section>
            </>
        }
    }
}
