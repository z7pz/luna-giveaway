macro_rules! config {
    ($($name:ident $type:tt $( = $value: expr)?)* ) => {
        lazy_static! {
            $(
                pub static ref $name: $type = std::env::var(stringify!($name)).unwrap_or_else(|_| {
                    $( if true { return $value.to_string(); } )?
                    panic!("coudn't find {} in env, or you should set default value.", stringify!($name))
                }).parse::<$type>().unwrap();
            )+
        }
    };
}

config! {
	// server
    PORT u32 = 3000 // default port 3000
    
	// discord
    DEFAULT_PREFIX String = "!"
    DISCORD_SECRET String = ""
    DISCORD_TOKEN String = "MTI0ODAyNDk4MzMwNTk4MTk2Mg.GMyrcS.zaWcvrrLizzWZ5nBdDUJtJNluRRa0EDpLRB_-U"
    DISCORD_ID String = ""
    REDIRECT_URI String = "http://localhost:3000/discord/redirect"
    GRANT_TYPE String = "authorization_code"
    
	// encryption
	SECRET String = "super-secret"
}
