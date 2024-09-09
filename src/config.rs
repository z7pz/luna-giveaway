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
    PORT u32 = 5000 // default port 3000
    
	// discord
    DEFAULT_PREFIX String = "!"
    DISCORD_SECRET String = "QrJXaz7wZhzs9HFm0ZiP8ohVP9rr-N3N"
    DISCORD_TOKEN String = "MTI0ODAyNDk4MzMwNTk4MTk2Mg.GMyrcS.zaWcvrrLizzWZ5nBdDUJtJNluRRa0EDpLRB_-U"
    DISCORD_ID String = "1248024983305981962"
    REDIRECT_URI String = "http://localhost:5000/discord/redirect"
    GRANT_TYPE String = "authorization_code"
    SCOPES String = "identify,guilds"
    
	// encryption
	SECRET String = "super-secret"

    FRONTEND_URI String = "http://localhost:3000"
}
