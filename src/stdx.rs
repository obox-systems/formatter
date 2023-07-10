#![allow(dead_code)]

pub(crate) fn cli() -> CommandBuilder<0> {
    CommandBuilder::new()
}

pub(crate) struct CommandBuilder<const N: usize> {
    commands: [wca::Command; N],
    handlers: [(String, wca::Routine); N],
}

#[derive(Clone)]
pub(crate) struct Property<'a> {
    pub(crate) name: &'a str,
    pub(crate) hint: &'a str,
    pub(crate) tag: wca::Type,
}

impl CommandBuilder<0> {
    fn new() -> Self {
        Self {
            handlers: [],
            commands: [],
        }
    }
}

pub(crate) trait CommandExt<T>: Sized {
    fn arg(self, hint: &str, tag: wca::Type) -> Builder<Self> {
        Builder::new(self).arg(hint, tag)
    }

    fn properties<const N: usize>(self, properties: [Property; N]) -> Builder<Self> {
        Builder::new(self).properties(properties)
    }
}

pub(crate) struct Builder<F> {
    handler: F,
    command: wca::Command,
}

impl<F> Builder<F> {
    fn new(handler: F) -> Self {
        let name = itertools::join(name::<F>().split('_'), ".");

        Self {
            handler,
            command: wca::Command::former().phrase(name).form(),
        }
    }

    pub(crate) fn arg(mut self, hint: &str, tag: wca::Type) -> Self {
        self.command
            .subjects
            .push(wca::grammar::settings::ValueDescription {
                hint: hint.into(),
                kind: tag,
                optional: false,
            });
        self
    }

    pub(crate) fn properties<const N: usize>(mut self, properties: [Property; N]) -> Self {
        for property in properties {
            self.command.properties.insert(
                property.name.to_owned(),
                wca::grammar::settings::ValueDescription {
                    hint: property.hint.to_owned(),
                    kind: property.tag,
                    optional: true,
                },
            );
        }
        self
    }
}

impl<F: Fn(T, wca::Args, wca::Props) -> crate::Result, T> CommandExt<T> for F {}

pub(crate) trait IntoBuilder<F>: Sized {
    fn into_builder(self) -> Builder<F>;
}

impl<F> IntoBuilder<F> for Builder<F> {
    fn into_builder(self) -> Self {
        self
    }
}

impl<F: Fn(wca::Context, wca::Args, wca::Props) -> crate::Result> IntoBuilder<F> for F {
    fn into_builder(self) -> Builder<F> {
        Builder::new(self)
    }
}

impl<const LEN: usize> CommandBuilder<LEN> {
    pub(crate) fn command<F: Fn(wca::Context, wca::Args, wca::Props) -> crate::Result + 'static>(
        self,
        command: impl IntoBuilder<F>,
    ) -> CommandBuilder<{ LEN + 1 }> {
        let Builder { handler, command } = command.into_builder();

        let handler = wca::Routine::new_with_ctx(move |(args, props), cx| {
            handler(cx, args, props).map_err(|report| wca::BasicError::new(format!("{report:?}")))
        });

        CommandBuilder {
            handlers: array_push(self.handlers, (command.phrase.clone(), handler)),
            commands: array_push(self.commands, command),
        }
    }

    pub(crate) fn build(self) -> wca::CommandsAggregator {
        wca::CommandsAggregator::former()
            .grammar(self.commands)
            .executor(self.handlers)
            .build()
    }
}

fn array_push<const N: usize, T>(this: [T; N], item: T) -> [T; N + 1] {
    use std::mem::MaybeUninit;

    unsafe {
        let mut uninit = MaybeUninit::<[T; N + 1]>::uninit();

        let ptr = uninit.as_mut_ptr() as *mut T;
        (ptr as *mut [T; N]).write(this);
        (ptr.add(N) as *mut [T; 1]).write([item]);

        uninit.assume_init()
    }
}

#[macro_export]
macro_rules! static_assert_size {
    ($ty:ty, $size:expr) => {
        const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    };
}

fn name<T>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.rfind(':').map_or(name, |tail| &name[tail + 1..])
}
