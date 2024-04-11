use rustler::{self, Binary, Env, NewBinary, Term};

#[rustler::nif]
pub fn to_pickle(term: Term) -> Binary {
    let de = rustler::serde::Deserializer::from(term);

    let opts = serde_pickle::SerOptions::new();

    // PROTO, version 3
    let mut output = vec![0x80, 0x3];

    let mut se = serde_pickle::Serializer::new(&mut output, opts);

    serde_transcode::transcode(de, &mut se).unwrap();

    // STOP
    output.push(b'.');

    let mut out = NewBinary::new(term.get_env(), output.len());
    out.as_mut_slice().clone_from_slice(&output);

    out.into()
}

#[rustler::nif]
pub fn from_pickle<'a>(env: Env<'a>, bin: Binary<'a>) -> Term<'a> {
    let opts = serde_pickle::DeOptions::new();
    let mut de = serde_pickle::Deserializer::new(bin.as_slice(), opts);
    let se = rustler::serde::Serializer::from(env);

    serde_transcode::transcode(&mut de, se).unwrap()
}

rustler::init!("pickler", [to_pickle, from_pickle]);
