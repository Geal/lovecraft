use std::panic;

/// restore the Lovecraft quotes on panic, as it should be
pub fn invoke() {
    panic_quotes(
        &LOVECRAFT_QUOTES,
        "You've met with a terrible fate, haven't you?",
    );
}

/// use panic quotes to your own taste
pub fn panic_quotes(quotes: &'static [&'static str], default: &'static str) {
    panic::set_hook(Box::new(move |panic_info| {
        match panic_info.payload().downcast_ref::<&str>() {
            None => println!("{}", default),
            Some(msg) => {
                let hash = msg.len();
                let quote = quotes[hash % quotes.len()];
                println!("{}\n\n{}", msg, quote);
            }
        }
    }));
}

const LOVECRAFT_QUOTES: [&'static str; 10] = [
    "It was from the artists and poets that the pertinent answers came, and I
know that panic would have broken loose had they been able to compare notes.
As it was, lacking their original letters, I half suspected the compiler of
having asked leading questions, or of having edited the correspondence in
corroboration of what he had latently resolved to see.",
    "There are not many persons who know what wonders are opened to them in the
stories and visions of their youth; for when as children we listen and dream,
we think but half-formed thoughts, and when as men we try to remember, we are
dulled and prosaic with the poison of life. But some of us awake in the night
with strange phantasms of enchanted hills and gardens, of fountains that sing
in the sun, of golden cliffs overhanging murmuring seas, of plains that stretch
down to sleeping cities of bronze and stone, and of shadowy companies of heroes
that ride caparisoned white horses along the edges of thick forests; and then
we know that we have looked back through the ivory gates into that world of
wonder which was ours before we were wise and unhappy.",
    "Instead of the poems I had hoped for, there came only a shuddering blackness
and ineffable loneliness; and I saw at last a fearful truth which no one had
ever dared to breathe before — the unwhisperable secret of secrets — The fact
that this city of stone and stridor is not a sentient perpetuation of Old New
York as London is of Old London and Paris of Old Paris, but that it is in fact
quite dead, its sprawling body imperfectly embalmed and infested with queer
animate things which have nothing to do with it as it was in life.",
    "The ocean ate the last of the land and poured into the smoking gulf, thereby
giving up all it had ever conquered. From the new-flooded lands it flowed
again, uncovering death and decay; and from its ancient and immemorial bed it
trickled loathsomely, uncovering nighted secrets of the years when Time was
young and the gods unborn. Above the waves rose weedy remembered spires. The
moon laid pale lilies of light on dead London, and Paris stood up from its damp
grave to be sanctified with star-dust. Then rose spires and monoliths that were
weedy but not remembered; terrible spires and monoliths of lands that men never
knew were lands...",
    "There was a night when winds from unknown spaces whirled us irresistibly into
limitless vacuum beyond all thought and entity. Perceptions of the most
maddeningly untransmissible sort thronged upon us; perceptions of infinity
which at the time convulsed us with joy, yet which are now partly lost to my
memory and partly incapable of presentation to others.",
    "You've met with a terrible fate, haven't you?",
    "You've met with a terrible fate, haven't you?",
    "You've met with a terrible fate, haven't you?",
    "You've met with a terrible fate, haven't you?",
    "You've met with a terrible fate, haven't you?",
];
