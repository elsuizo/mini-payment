use chrono::DateTime;
use chrono::Utc;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
struct UserName<const N: usize>(String);

pub struct User<const N: usize> {
    name: UserName<N>,
    bird_date: DateTime<Utc>,
}

impl<const N: usize> UserName<N> {
    pub fn inner(self) -> String {
        // el que llama a esta funcion obtiene la `String` que esta dentro del type pero no posee
        // mas `SubscriberName` porque esta funcion toma como parametro `self`, consumiendolo de
        // acuerdo a las semanticas de move
        self.0
    }

    pub fn inner_ref(&self) -> &str {
        // el que llama a esta funcion obtiene una referencia a la string que esta adentro del
        // type. Esto le da al que llama a esta funcion acceso solo de lectura al elemento que esta
        // dentro de `SubscriberName`
        &self.0
    }
    /// retorna una instancia de `SubscriberName` si el input satisface todas nuestras validaciones
    /// sobre los nombres de los subscriptores
    pub fn parse(s: String) -> Result<Self, String> {
        // `.trim()` retorna una vista del input `s` sin ningun espacio en blanco, tambien
        // verificamos que no este vacia
        let is_empty_or_whitespace = s.trim().is_empty();

        // un grafeno retorna un iterador sobre los grafenos en la entrada `s`, el parametro `true`
        // significa que queremos usar el conjunto de grafenos extendidos(que es el recomendado)
        let is_too_long = s.graphemes(true).count() > N;

        // iteramos sobre todos los caracteres en la entrada `s` para chequear si cualquiera de
        // ellos matchea uno de los caracteres en el array de caracteres prohibidos
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        let contains_forbiddden_characteres = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbiddden_characteres {
            Err(format!("{} is not a valid subscriber name!!!", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl<const N: usize> AsRef<str> for UserName<N> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
