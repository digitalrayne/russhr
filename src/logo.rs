
const LOGO: &'static str = "                                      \x1B[1m\u{2591}\x1B[0m                           \x1B[1m\u{2592}\x1B[0m
       \x1B[31m\u{2584}\u{2584}\u{2584}\u{2584}\u{2584}\u{2588}\x1B[37m             \x1B[31m\u{2584}\x1B[37m          \x1B[1m\u{2591}\u{2588}\u{2591}\x1B[0m              \x1B[31m\u{2584}\x1B[37m \x1B[31m\u{2584}\x1B[37m     \x1B[31m\u{2584}\u{2588}\x1B[37m \x1B[1m\u{2592}\u{2588}\u{2592}\x1B[0m     \x1B[1;31m\u{2584}\u{2584}\x1B[0;31m\u{2584}\u{2584}\x1B[37m
     \x1B[31m\u{2584}\x1B[1;41m\u{2584}\x1B[40m\u{2588}\x1B[41m\u{2580}\u{2580}\u{2580}\x1B[0;31m\u{2588}\u{2588}\x1B[37m  \x1B[31m\u{2584}\x1B[1m\u{2588}\x1B[0;31m\u{2584}\u{2584}\x1B[37m  \x1B[31m\u{2584}\u{2584}\x1B[1m\u{2584}\x1B[0;31m\u{2588}\u{2588}\u{2580}\x1B[37m  \x1B[31m\u{2584}\u{2584}\u{2584}\x1B[1m\u{2584}\x1B[41m\u{2584}\u{2584}\u{2584}\x1B[40m\u{2584}\x1B[0;31m\u{2584}\x1B[1;37m\u{2591}\x1B[0m    \x1B[31m\u{2584}\x1B[1m\u{2584}\x1B[41m\u{2584}\x1B[40m\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m \x1B[31m\u{2588}\x1B[1;41m\u{2584}\x1B[0;31m\u{2588}\x1B[37m   \x1B[31m\u{2588}\x1B[1;41m\u{2584}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[37m\u{2592}\u{2588}\u{2588}\u{2588}\u{2592}\x1B[0;31m\u{2584}\x1B[1m\u{2588}\u{2588}\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\x1B[37m
  \x1B[31m\u{2584}\u{2584}\x1B[1;41m\u{2584}\x1B[40m\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\u{2591}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m  \x1B[31m\u{2584}\x1B[1m\u{2588}\u{2588}\x1B[0;31m\u{2588}\x1B[37m  \x1B[31m\u{2588}\x1B[1m\u{2588}\u{2588}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[0m  \x1B[31m\u{2584}\x1B[1;41m\u{2584}\x1B[40m\u{2588}\x1B[41m\u{2580}\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\u{2580}\u{2580}\x1B[37m  \x1B[31m\u{2584}\u{2588}\x1B[1;41m\u{2584}\x1B[40m\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\u{2580}\u{2580}\u{2580}\u{2588}\x1B[37m \x1B[31m\u{2588}\x1B[1m\u{2588}\u{2588}\x1B[0;31m\u{2588}\x1B[37m   \x1B[1;31;41m\u{2584}\x1B[40m\u{2588}\u{2591}\u{2591}\x1B[0m \x1B[1m\u{2592}\u{2588}\u{2592}\x1B[31;41m\u{2584}\x1B[40m\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m
  \x1B[31m\u{2588}\x1B[1;41m\u{2580}\u{2580}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\u{2584}\u{2584}\u{2588}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m \x1B[31m\u{2588}\x1B[1m\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[0m \x1B[31m\u{2588}\x1B[1m\u{2588}\u{2588}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[0m \x1B[31m\u{2588}\x1B[1m\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0;31m\u{2584}\u{2584}\x1B[37m  \x1B[31m\u{2580}\u{2588}\x1B[1m\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0;31m\u{2584}\u{2584}\u{2584}\x1B[37m \x1B[31m\u{2580}\x1B[37m \x1B[1;31;41m\u{2584}\x1B[40m\u{2588}\u{2588}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2584}\u{2584}\u{2588}\u{2588}\u{2591}\u{2591}\x1B[0;31m\u{2584}\x1B[37m \x1B[1m\u{2592}\x1B[0;31m\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\u{2584}\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[0m
   \x1B[31m\u{2580}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m  \x1B[1;31m\u{2588}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0;31m\u{2584}\x1B[1m\u{2588}\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[0m \x1B[31m\u{2588}\x1B[1;41m\u{2580}\x1B[0;31m\u{2580}\u{2580}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2584}\x1B[37m \x1B[31m\u{2580}\u{2580}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\x1B[37m  \x1B[1;31m\u{2588}\u{2588}\x1B[0;31m\u{2588}\u{2588}\x1B[1;41m\u{2580}\u{2580}\u{2580}\u{2580}\x1B[40m\u{2588}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[0m    \x1B[31m\u{2580}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m
    \x1B[31m\u{2584}\x1B[1;41m\u{2584}\x1B[40m\u{2588}\x1B[0;31m\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m  \x1B[1;31m\u{2588}\x1B[0;31m\u{2588}\u{2588}\x1B[1;41m\u{2580}\u{2580}\u{2580}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m \x1B[31m\u{2580}\u{2588}\u{2580}\x1B[37m     \x1B[31m\u{2580}\x1B[1;41m\u{2584}\x1B[40m\u{2588}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[0m      \x1B[31m\u{2580}\x1B[1m\u{2588}\u{2588}\u{2588}\x1B[0;31m\u{2588}\u{2588}\u{2584}\x1B[37m \x1B[1;31m\u{2588}\u{2588}\x1B[0;31m\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m      \x1B[1;31m\u{2584}\x1B[41m\u{2584}\u{2584}\x1B[0;31m\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m
   \x1B[31m\u{2584}\x1B[1m\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\x1B[1;41m\u{2584}\x1B[40m\u{2588}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m  \x1B[31m\u{2588}\x1B[1;41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\u{2591}\x1B[0m   \x1B[31m\u{2584}\u{2584}\x1B[1m\u{2584}\u{2584}\u{2584}\u{2588}\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m \x1B[31m\u{2588}\u{2588}\u{2584}\x1B[1m\u{2584}\u{2584}\x1B[41m\u{2584}\x1B[40m\u{2588}\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[41m\u{2584}\x1B[40m\u{2588}\u{2588}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[0m  \x1B[31m\u{2580}\x1B[1;41m\u{2584}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0;31m\u{2588}\x1B[37m  \x1B[31m\u{2584}\x1B[1m\u{2584}\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m
 \x1B[31m\u{2580}\u{2588}\x1B[1m\u{2588}\x1B[0;31m\u{2588}\u{2588}\u{2588}\x1B[37m  \x1B[1;31m\u{2588}\x1B[41m\u{2580}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\u{2591}\x1B[0m  \x1B[31m\u{2588}\u{2580}\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m    \x1B[31m\u{2584}\u{2588}\u{2588}\x1B[1;41m\u{2580}\u{2580}\u{2580}\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m   \x1B[31m\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1;41m\u{2580}\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m \x1B[1;31m\u{2588}\u{2588}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m   \x1B[1;31m\u{2588}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m \x1B[31m\u{2580}\x1B[1;41m\u{2580}\u{2580}\u{2580}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0;31m\u{2580}\u{2588}\u{2588}\x1B[1m\u{2591}\u{2591}\x1B[0m
  \x1B[31m\u{2588}\u{2588}\u{2588}\u{2580}\x1B[37m   \x1B[1;31;41m\u{2580}\x1B[0;31m\u{2588}\u{2588}\u{2580}\u{2588}\x1B[37m     \x1B[1;31;41m\u{2584}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[0m     \x1B[31m\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2580}\u{2580}\x1B[37m    \x1B[31m\u{2588}\x1B[1m\u{2591}\x1B[0;31m\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m \x1B[1;31m\u{2588}\x1B[0;31m\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m   \x1B[1;31m\u{2580}\x1B[0;31m\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0m  \x1B[31m\u{2580}\u{2588}\u{2588}\x1B[1m\u{2591}\u{2591}\x1B[0m  \x1B[31m\u{2588}\u{2588}\u{2588}\x1B[1m\u{2591}\x1B[0;31m\u{2584}\x1B[37m
  \x1B[1;31m\u{2588}\x1B[0;31m\u{2584}\x1B[1m\u{2591}\x1B[0m   \x1B[1;31;41m\u{2580}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[0;31m\u{2588}\x1B[37m \x1B[1;31m\u{2588}\x1B[0;31m\u{2588}\x1B[37m  \x1B[1m\u{2591}\x1B[0m  \x1B[31m\u{2588}\x1B[1m\u{2591}\x1B[0m    \x1B[1;31m\u{2584}\x1B[0;31m\u{2588}\u{2588}\u{2580}\x1B[37m          \x1B[31m\u{2588}\x1B[1m\u{2591}\x1B[0;31m\u{2580}\x1B[37m    \x1B[31m\u{2580}\u{2588}\x1B[1m\u{2591}\x1B[0m  \x1B[31m\u{2580}\u{2588}\u{2588}\x1B[1m\u{2591}\u{2591}\x1B[0m    \x1B[31m\u{2580}\u{2588}\x1B[1m\u{2591}\x1B[0m    \x1B[31m\u{2588}\x1B[1m\u{2591}\x1B[0m   \x1B[31m\u{2580}\u{2580}\u{2588}\x1B[1m\u{2591}\x1B[0m
  \x1B[31m\u{2580}\u{2580}\x1B[37m    \x1B[31m\u{2580}\x1B[37m \x1B[1;31m\u{2588}\x1B[0;31m\u{2580}\x1B[37m \x1B[31m\u{2588}\x1B[1m\u{2591}\x1B[0m \x1B[1m\u{2591}\u{2588}\u{2591}\x1B[0m \x1B[31m\u{2580}\x1B[37m     \x1B[1;31m\u{2588}\x1B[0;31m\u{2588}\x1B[37m            \x1B[31m\u{2580}\u{2580}\x1B[37m      \x1B[1;31;41m\u{2580}\x1B[0;31m\u{2588}\x1B[37m   \x1B[31m\u{2580}\u{2588}\u{2588}\x1B[37m      \x1B[31m\u{2588}\x1B[1m\u{2591}\x1B[0m           \x1B[31m\u{2588}\x1B[1m\u{2591}\x1B[0m
          \x1B[1;31m\u{2588}\x1B[0;31m\u{2588}\x1B[37m   \x1B[1m\u{2591}\u{2588}\u{2588}\u{2588}\u{2591}\x1B[0m      \x1B[1;31m\u{2580}\x1B[0;31m\u{2588}\x1B[1m\u{2591}\x1B[0m                   \x1B[31m\u{2580}\u{2580}\x1B[37m   \x1B[1;31m\u{2584}\x1B[0;31m\u{2588}\u{2588}\x1B[37m       \x1B[1;31m\u{2591}\x1B[0m          \x1B[1;31m\u{2584}\x1B[0;31m\u{2588}\u{2588}\x1B[37m
 \x1B[30m\u{2588}\u{2588}\u{2588}\u{2580}\x1B[1;33m\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\x1B[37m\u{2591}\u{2588}\u{2591}\x1B[33m\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550} v";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const LOGOTAIL: &'static str = " by ec0 <3 \u{2550}\u{2550}\u{2550}\u{2550}\x1B[0;31m\u{2588}\u{2588}\x1B[1;33m\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\x1B[0;31m\u{2580}\u{2588}\u{2580}\x1B[37m
     \x1B[30m\u{2568}\u{2580}\u{2580}\u{2580}\x1B[37m        \x1B[1m\u{2591}\x1B[0m";

pub fn display_logo() {
    eprintln!("{}{}{}", LOGO, VERSION, LOGOTAIL);
}