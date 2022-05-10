
// match_tokesn and match_id are from:
//  https://danielkeep.github.io/tlborm/book/mbe-min-captures-and-expansion-redux.html
//macro_rules! match_tokens {
//    ($a:tt + $b:tt) => {"got an addition"};
//    (($i:ident)) => {"got an identifier"};
//    ($($other:tt)*) => {"got something else"};
//}
//
//macro_rules! match_id {
//    ($i:ident) => {"got an identifier"};
//}

// My say_something
// Repitition characters
// ? --- zero or one
// + -- one or more
// * -- any number
#[allow(unused)]
macro_rules! say_something {
    // Zero parameters
    () => {
       println!("no-parameters");
    };
    ($i:ident) => {
        log_syntax($i)
        println!("one-parameter: {}", stringify!($i));
    };
    // One or more parameters
    ( $($i:ident) *) => {
        print!("one or more parameter: ");
        $(
            print!("{} ", stringify!($i));
        )+
        println!();
    };
}

macro_rules! SmStates {
    (
        $(
            $name:ident,
        )+
    ) => {
        #[allow(unused)]
        #[derive(Debug)]
        enum States {
            $(
                $name,
            )+
        }
    }
}

macro_rules! ProtocolMsgs {
    (
        $pname:ident {
            $(
                $mname:ident {
                    $(
                        $fname:ident: $ftype:ty,
                    )*
                },
            )+
        }
    ) => {
        #[allow(unused)]
        #[derive(Debug)]
        enum $pname {
            $(
                $mname {
                    $(
                        $fname: $ftype,
                    )*
                },
            )+
        }
    }
}

macro_rules! StateMachine {
    ($name:ident) => {
        // Needs moved to StateMachine!
        SmStates!{
            Start,
            Work,
            Stopping,
            Stopped,
        }

        // Needs moved to StateMachine!
        ProtocolMsgs!{
            Protocol1 {
                Msg1 {
                    f1: i32,
                },
                Msg2 {
                    f1: i32,
                },
            }
        }

        #[allow(unused)]
        #[derive(Debug)]
        struct $name {
            current_state: States,
            data1: i32,
        }
    }
}

StateMachine! {
    MySm
    //{
    //  // Should be here
    //  SmStates!{
    //      Start,
    //      Work,
    //      Stopping,
    //      Stopped,
    //  }
    //
    //  // Should be here
    //  ProtocolMsgs!(Protocol1) {
    //          Msg1 {
    //              f1: i32,
    //          },
    //          Msg2 {
    //              f1: i32,
    //          },
    //      }
    //  }
    //
    //  StateData!{
    //      data1: i32,
    //  }
    //     
    //  // All State! methods receive two parameters (sm: StateData, msg: Protocol1)
    //  State!(Start) {
    //      // Optional Entry, called once when this state first the current_state
    //      fn entry(self, sm: StateData, msg: Protocol1) {
    //          match msg {
    //              Protocol1::Msg1 { f1 } => {
    //                  println!("msg.f1={:?}", msg)
    //              }
    //          }
    //      }
    //
    //      // Required, called on every message while this is the current state
    //      fn process_msg(self, sm: StateData, msg: Protocol1) {
    //          match msg {
    //              Protocol1::Msg1 { f1 } => {
    //                  println!("msg.f1={:?}", msg)
    //              }
    //          }
    //      }
    //
    //      // Optional Exit, called once when this state is going to be replaced as cureentfirst
    //      fn exit(self, sm: StateData, msg: Protocol1) {
    //          match msg {
    //              Protocol1::Msg1 { f1 } => {
    //                  println!("msg.f1={:?}", msg)
    //              }
    //          }
    //      }
    //  }
    //}
}

fn main() {
    //let x = match_tokens!((hello));
    //println!("{}", x);
    //let y = match_id!(hello);
    //println!("{}", y);

    //say_something!{}
    //say_something!(one);
    //say_something!(dude cool);

    let mut mysm = MySm {
        current_state: States::Start,
        data1: 0,
    };
    mysm.data1 += 1;
    println!("{:?}", mysm);
}
