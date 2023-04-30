use std::marker::PhantomData;

struct Created;
struct Submitted;
struct Completed;
struct Failed;
struct Canceled;

#[derive(Debug)]
enum TaskError {
    InvalidCommand
}

//enum ClientCommands {
//    Created(ClientCommand<Created>),
//    Submitted(ClientCommand<Submitted>),
//    Completed(ClientCommand<Completed>),
//    Failed(ClientCommand<Failed>),
//    Canceled(ClientCommand<Canceled>),
//}

struct ClientCommand<State = Created> {
    id: uuid::Uuid,
    args: Vec<String>,
    state: PhantomData<State>,
}

impl ClientCommand<Created> {
    pub fn submit(self) -> ClientCommand<Submitted> {
        ClientCommand {
            id: self.id,
            args: self.args,
            state: PhantomData,
        }
    }

    pub fn cancel(self) -> ClientCommand<Canceled> {
        ClientCommand {
            id: self.id,
            args: self.args,
            state: PhantomData,
        }
    }
}

impl ClientCommand<Submitted> {
    pub fn complete(self) -> Result<ClientCommand<Completed>, ClientCommand<Failed>> {
        Ok(ClientCommand {
            id: self.id,
            args: self.args,
            state: PhantomData::<Completed>
        })

    }
}

impl ClientCommand {
    pub fn new(args: Vec<String>) -> Result<Self, TaskError> {
        match args.is_empty() {
            true => Err(TaskError::InvalidCommand),
            false => Ok(ClientCommand {
                id: uuid::Uuid::new_v4(),
                args: args,
                state: PhantomData,
            })
        }
    }
}


// Impl Display for all possible States
macro_rules! impl_display_for_command {
    ($command_name:ident) => {
        impl<State> std::fmt::Display for $command_name<State> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{} {} [{}]",
                    //stringify!($command_name),
                    self.id,
                    std::any::type_name::<State>()
                        .rsplit("::")
                        .next()
                        .unwrap_or("<unknown type>"),
                    self.args.join(", "),
                )
            }
        }
    };
}
impl_display_for_command!(ClientCommand);

fn main() -> Result<(), TaskError> {
	let cmd1 = ClientCommand::new(vec!("whoami".into(),  "/all".into()))?;

	// Won't compile. Can neither submit+cancel or cancel+submit.
	//let cmd1 = cmd1.cancel();

	let cmd1 = cmd1.submit();

	match cmd1.complete() {
		Ok(i) => println!("Task completed: {}", i),
		Err(e) => println!("Task failed: {}", e)
	}

    Ok(())
}
