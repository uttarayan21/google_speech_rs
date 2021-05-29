use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
#[derive(Debug)]
pub struct Speech {
    pyobject_speech: PyObject,
}
impl Speech {
    /// Create a new speech object with language
    pub fn new<S: AsRef<str>>(text: S, lang: Lang) -> PyResult<Self> {
        Python::with_gil(|py| {
            let google_speech = PyModule::import(py, "google_speech").unwrap();
            let pyobject_speech: PyObject = google_speech
                .call1("Speech", (text.as_ref(), lang.to_string()))
                .unwrap()
                .extract()
                .unwrap();
            Ok(Self { pyobject_speech })
        })
    }

    /// Play the speech object
    pub fn play(&self) -> PyResult<()> {
        Python::with_gil(|py| {
            self.pyobject_speech.call_method0(py, "play")?;
            Ok(())
        })
    }

    /// # Safety
    /// The save function is a wrapper around google_speech.Speech.save
    /// which might be unsafe
    pub unsafe fn save<S: AsRef<str>>(&self, path: S) -> PyResult<()> {
        Python::with_gil(|py| {
            self.pyobject_speech
                .call_method1(py, "save", (path.as_ref(),))?;
            Ok(())
        })
    }

    // /// Change the language
    // pub fn lang(&mut self, lang: Lang) -> PyResult<()> {
    //     Python::with_gil(|py| {
    //         // self.pyobject_speech.getattr
    //         // let globals =  PyDict::
    //         Python::run(
    //             py,
    //             // &format!("speech.lang = \"{}\"", lang.to_string()),
    //             &format!("print(\"{}\")", lang.to_string()),
    //             Some(vec![("speech", lang.to_string().as_str())].into_py_dict(py)),
    //             None,
    //         )?;
    //         println!("{:?}", self.pyobject_speech);
    //         Ok(())
    //     })
    // }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[non_exhaustive]
pub enum Lang {
    af,
    ar,
    bn,
    bs,
    ca,
    cs,
    cy,
    da,
    de,
    el,
    en,
    en_au,
    en_ca,
    en_gb,
    en_gh,
    en_ie,
    en_in,
    en_ng,
    en_nz,
    en_ph,
    en_tz,
    en_uk,
    en_us,
    en_za,
    eo,
    es,
    es_es,
    es_us,
    et,
    fi,
    fr,
    fr_ca,
    fr_fr,
    hi,
    hr,
    hu,
    hy,
    id,
    is,
    it,
    ja,
    jw,
    km,
    ko,
    la,
    lv,
    mk,
    ml,
    mr,
    my,
    ne,
    nl,
    no,
    pl,
    pt,
    pt_br,
    pt_pt,
    ro,
    ru,
    si,
    sk,
    sq,
    sr,
    su,
    sv,
    sw,
    ta,
    te,
    th,
    tl,
    tr,
    uk,
    vi,
    zh_cn,
    zh_tw,
}

impl ToString for Lang {
    fn to_string(&self) -> String {
        let ret = match self {
            Lang::af => "af",
            Lang::ar => "ar",
            Lang::bn => "bn",
            Lang::bs => "bs",
            Lang::ca => "ca",
            Lang::cs => "cs",
            Lang::cy => "cy",
            Lang::da => "da",
            Lang::de => "de",
            Lang::el => "el",
            Lang::en => "en",
            Lang::en_au => "en-au",
            Lang::en_ca => "en-ca",
            Lang::en_gb => "en-gb",
            Lang::en_gh => "en-gh",
            Lang::en_ie => "en-ie",
            Lang::en_in => "en-in",
            Lang::en_ng => "en-ng",
            Lang::en_nz => "en-nz",
            Lang::en_ph => "en-ph",
            Lang::en_tz => "en-tz",
            Lang::en_uk => "en-uk",
            Lang::en_us => "en-us",
            Lang::en_za => "en-za",
            Lang::eo => "eo",
            Lang::es => "es",
            Lang::es_es => "es-es",
            Lang::es_us => "es-us",
            Lang::et => "et",
            Lang::fi => "fi",
            Lang::fr => "fr",
            Lang::fr_ca => "fr-ca",
            Lang::fr_fr => "fr-fr",
            Lang::hi => "hi",
            Lang::hr => "hr",
            Lang::hu => "hu",
            Lang::hy => "hy",
            Lang::id => "id",
            Lang::is => "is",
            Lang::it => "it",
            Lang::ja => "ja",
            Lang::jw => "jw",
            Lang::km => "km",
            Lang::ko => "ko",
            Lang::la => "la",
            Lang::lv => "lv",
            Lang::mk => "mk",
            Lang::ml => "ml",
            Lang::mr => "mr",
            Lang::my => "my",
            Lang::ne => "ne",
            Lang::nl => "nl",
            Lang::no => "no",
            Lang::pl => "pl",
            Lang::pt => "pt",
            Lang::pt_br => "pt-br",
            Lang::pt_pt => "pt-pt",
            Lang::ro => "ro",
            Lang::ru => "ru",
            Lang::si => "si",
            Lang::sk => "sk",
            Lang::sq => "sq",
            Lang::sr => "sr",
            Lang::su => "su",
            Lang::sv => "sv",
            Lang::sw => "sw",
            Lang::ta => "ta",
            Lang::te => "te",
            Lang::th => "th",
            Lang::tl => "tl",
            Lang::tr => "tr",
            Lang::uk => "uk",
            Lang::vi => "vi",
            Lang::zh_cn => "zh-cn",
            Lang::zh_tw => "zh-tw",
        };
        ret.to_string()
    }
}
