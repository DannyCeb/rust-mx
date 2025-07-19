use std::io::{self, Cursor, Read, Write};

/// Estructura de datos para compartir información serializada
///
/// Contiene campos de diferentes tipos que pueden ser convertidos a un formato binario
/// y posteriormente reconstruidos. Útil para comunicación entre sistemas o persistencia.
#[derive(Debug)]
pub struct Data {
    pub field1: u32,
    pub field2: u16,
    pub field3: String,
}

impl Data {
    // convierte de struct a [u8]
    // [1010_1000_0000_0000_1111_1000_0000_0000_0000_0000_1111_1000_1010_1000_0000_0000_1111_1000_0000_0000_1010_1000_0000_0000_1111_1000_0000_0000.........]
    // |----------------field1-----------------|-------field2------|------------field3.len()---------------|--------------field3_u8-------------------------|
    //                32 bits                          16 bits                  32 bits                                   8 bits * field3.len()

    /// Serializa la estructura a un formato binario
    ///
    /// Formato del buffer:
    /// ```text
    /// [field1 (4 bytes)][field2 (2 bytes)][longitud field3 (4 bytes)][field3 (N bytes)]
    /// ```
    ///
    /// # Ejemplo
    /// ```
    /// let data = Data {
    ///     field1: 42,
    ///     field2: 7,
    ///     field3: "Hola".to_string(),
    /// };
    /// let bytes = data.serialize().unwrap();
    /// ```
    ///
    /// # Errores
    /// Devuelve `io::Error` si hay problemas escribiendo en el buffer interno
    ///
    ///
    pub fn serialize(&self) -> io::Result<Vec<u8>> {
        // Pre-asigna capacidad para optimizar
        let mut bytes = Vec::with_capacity(4 + 2 + 4 + self.field3.len());

        // Serializa field1 (u32) en formato nativo (4 bytes)
        bytes.write_all(&self.field1.to_ne_bytes())?;

        // Serializa field2 (u16) en formato nativo (2 bytes)
        bytes.write_all(&self.field2.to_ne_bytes())?;

        // Serializa longitud de field3 como u32 (4 bytes)
        let field3_len = self.field3.len() as u32;
        bytes.write_all(&field3_len.to_ne_bytes())?;

        // Serializa contenido de field3 (bytes crudos)
        bytes.extend_from_slice(self.field3.as_bytes());

        Ok(bytes)
    }

    /// Deserializa un buffer binario a una instancia de Data
    ///
    /// # Argumentos
    /// * `cursor` - Cursor mutable sobre el buffer de entrada
    ///
    /// # Ejemplo
    /// ```
    /// let buffer: &[u8] = ...; // Datos serializados
    /// let mut cursor = Cursor::new(buffer);
    /// let data = Data::deserialize(&mut cursor).unwrap();
    /// ```
    ///
    /// # Errores
    /// - `io::Error` si hay problemas de lectura o datos insuficientes
    /// - `InvalidData` si los bytes de texto no son UTF-8 válido
    pub fn deserialize(cursor: &mut Cursor<&[u8]>) -> io::Result<Data> {
        // Buffer para field1 (u32: 4 bytes)
        let mut field1_bytes = [0u8; 4];
        cursor.read_exact(&mut field1_bytes)?;
        let field1 = u32::from_ne_bytes(field1_bytes);

        // Buffer para field2 (u16: 2 bytes)
        let mut field2_bytes = [0u8; 2];
        cursor.read_exact(&mut field2_bytes)?;
        let field2 = u16::from_ne_bytes(field2_bytes);

        // Buffer para longitud de field3 (u32: 4 bytes)
        let mut len_bytes = [0u8; 4];
        cursor.read_exact(&mut len_bytes)?;
        let len = u32::from_ne_bytes(len_bytes) as usize;

        // Leer N bytes del string
        let mut field3_bytes = vec![0u8; len];
        cursor.read_exact(&mut field3_bytes)?;

        // Convertir a String con validación UTF-8
        let field3 = String::from_utf8(field3_bytes)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Datos UTF-8 inválidos"))?;

        Ok(Data {
            field1,
            field2,
            field3,
        })
    }
}
