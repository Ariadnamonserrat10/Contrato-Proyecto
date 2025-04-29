// #![no_std]

// use soroban_sdk::{contract, contractimpl, map, Env, String, Map, Symbol, symbol_short};

// #[contract]
// pub struct Contract;

<<<<<<< HEAD
#[contractimpl]
impl Contract {
    // pub fn insert_estudiante(env: Env, id: String, nombre: String, email: String) {
    //     let mut estudiantes: Map<String, Map<Symbol, String>> = env
    //         .storage()
    //         .persistent()
    //         .get(&symbol_short!("estd"))  // Cambiado a "estd"
    //         .unwrap_or_else(|| Map::new(&env));

    //     let estudiante_data = map![
    //         &env,
    //         (symbol_short!("nombre"), nombre),
    //         (symbol_short!("email"), email),
    //     ];

    //     estudiantes.set(id.clone(), estudiante_data);
    //     env.storage().persistent().set(&symbol_short!("estd"), &estudiantes);  // Cambiado a "estd"
    // }

    // pub fn edit_estudiante(env: Env, id: String, new_nombre: String, new_email: String) {
    //     let mut estudiantes: Map<String, Map<Symbol, String>> = env
    //         .storage()
    //         .persistent()
    //         .get(&symbol_short!("estd"))  // Cambiado a "estd"
    //         .unwrap_or_else(|| Map::new(&env));

    //     if let Some(mut estudiante_data) = estudiantes.get(id.clone()) {
    //         estudiante_data.set(symbol_short!("nombre"), new_nombre);
    //         estudiante_data.set(symbol_short!("email"), new_email);
    //         estudiantes.set(id.clone(), estudiante_data);
    //         env.storage().persistent().set(&symbol_short!("estd"), &estudiantes);  // Cambiado a "estd"
    //     }
    // }

    // pub fn delete_estudiante(env: Env, id: String) {
    //     let mut estudiantes: Map<String, Map<Symbol, String>> = env
    //         .storage()
    //         .persistent()
    //         .get(&symbol_short!("estd"))  // Cambiado a "estd"
    //         .unwrap_or_else(|| Map::new(&env));

    //     estudiantes.remove(id.clone());
    //     env.storage().persistent().set(&symbol_short!("estd"), &estudiantes);  // Cambiado a "estd"
    // }

    // pub fn get_estudiante(env: Env, id: String) -> Option<Map<Symbol, String>> {
    //     let estudiantes: Map<String, Map<Symbol, String>> = env
    //         .storage()
    //         .persistent()
    //         .get(&symbol_short!("estd"))  // Cambiado a "estd"
    //         .unwrap_or_else(|| Map::new(&env));

    //     estudiantes.get(id)
    // }

    // pub fn buscar_estudiantes(env: Env, nombre: String) -> Option<Map<Symbol, String>> {
    //     let estudiantes: Map<String, Map<Symbol, String>> = env
    //         .storage()
    //         .persistent()
    //         .get(&symbol_short!("estd"))  // Cambiado a "estd"
    //         .unwrap_or_else(|| Map::new(&env));

    //     estudiantes.iter().find(|(_, data)| {
    //         data.get(symbol_short!("nombre"))
    //             .map(|n| n == nombre) // Comparar String directamente
    //             .unwrap_or(false)
    //     }).map(|(_, data)| data.clone())
    // }



//    Funciones del curso

// pub fn registrar(env: Env, id_curso: String, materia: String, titulo: String, id_docente: String) {
//     let mut cursos: Map<String, Map<Symbol, String>> = env
//         .storage()
//         .persistent()
//         .get(&symbol_short!("curs"))
//         .unwrap_or_else(|| Map::new(&env));

//     let curso_data = map![
//         &env,
//         (symbol_short!("materia"), materia),
//         (symbol_short!("titulo"), titulo),
//         (symbol_short!("id_docente"), id_docente),
//     ];

//     cursos.set(id_curso.clone(), curso_data);
//     env.storage().persistent().set(&symbol_short!("curs"), &cursos);
// }

// pub fn eliminar(env: Env, id_curso: String) {
//     let mut cursos: Map<String, Map<Symbol, String>> = env
//         .storage()
//         .persistent()
//         .get(&symbol_short!("curs"))
//         .unwrap_or_else(|| Map::new(&env));

//     cursos.remove(id_curso.clone());
//     env.storage().persistent().set(&symbol_short!("curs"), &cursos);
// }

// pub fn buscar(env: Env, filtro: String) -> Vec<(String, Map<Symbol, String>)> {
//     let cursos: Map<String, Map<Symbol, String>> = env
//         .storage()
//         .persistent()
//         .get(&symbol_short!("curs"))
//         .unwrap_or_else(|| Map::new(&env));

//     cursos
//         .iter()
//         .filter(|(id, data)| {
//             // Buscar coincidencias en ID
//             id.contains(&filtro) ||
//             // Buscar coincidencias en materia
//             data.get(symbol_short!("materia"))
//                 .map(|m| m.contains(&filtro))
//                 .unwrap_or(false) ||
//             // Buscar coincidencias en título
//             data.get(symbol_short!("titulo"))
//                 .map(|t| t.contains(&filtro))
//                 .unwrap_or(false) ||
//             // Buscar coincidencias en ID del docente
//             data.get(symbol_short!("id_docente"))
//                 .map(|d| d.contains(&filtro))
//                 .unwrap_or(false)
//         })
//         .map(|(id, data)| (id.clone(), data.clone()))
//         .collect()
// }

// pub fn actualizar(env: Env, id_curso: String, nuevos_datos: Map<Symbol, String>) {
//     let mut cursos: Map<String, Map<Symbol, String>> = env
//         .storage()
//         .persistent()
//         .get(&symbol_short!("curs"))
//         .unwrap_or_else(|| Map::new(&env));

//     if let Some(mut curso_data) = cursos.get(id_curso.clone()) {
//         // Actualizar cada campo si está presente en nuevos_datos
//         if let Some(materia) = nuevos_datos.get(symbol_short!("materia")) {
//             curso_data.set(symbol_short!("materia"), materia);
//         }
        
//         if let Some(titulo) = nuevos_datos.get(symbol_short!("titulo")) {
//             curso_data.set(symbol_short!("titulo"), titulo);
//         }
        
//         if let Some(id_docente) = nuevos_datos.get(symbol_short!("id_docente")) {
//             curso_data.set(symbol_short!("id_docente"), id_docente);
//         }
        
//         cursos.set(id_curso.clone(), curso_data);
//         env.storage().persistent().set(&symbol_short!("curs"), &cursos);
//     }
// }
   
}

mod test;


=======
// #[contractimpl]
// impl Contract {
//     // pub fn insert_estudiante(env: Env, id: String, nombre: String, email: String) {
//     //     let mut estudiantes: Map<String, Map<Symbol, String>> = env
//     //         .storage()
//     //         .persistent()
//     //         .get(&symbol_short!("estd"))  // Cambiado a "estd"
//     //         .unwrap_or_else(|| Map::new(&env));

//     //     let estudiante_data = map![
//     //         &env,
//     //         (symbol_short!("nombre"), nombre),
//     //         (symbol_short!("email"), email),
//     //     ];

//     //     estudiantes.set(id.clone(), estudiante_data);
//     //     env.storage().persistent().set(&symbol_short!("estd"), &estudiantes);  // Cambiado a "estd"
//     // }

//     // pub fn edit_estudiante(env: Env, id: String, new_nombre: String, new_email: String) {
//     //     let mut estudiantes: Map<String, Map<Symbol, String>> = env
//     //         .storage()
//     //         .persistent()
//     //         .get(&symbol_short!("estd"))  // Cambiado a "estd"
//     //         .unwrap_or_else(|| Map::new(&env));

//     //     if let Some(mut estudiante_data) = estudiantes.get(id.clone()) {
//     //         estudiante_data.set(symbol_short!("nombre"), new_nombre);
//     //         estudiante_data.set(symbol_short!("email"), new_email);
//     //         estudiantes.set(id.clone(), estudiante_data);
//     //         env.storage().persistent().set(&symbol_short!("estd"), &estudiantes);  // Cambiado a "estd"
//     //     }
//     // }

//     // pub fn delete_estudiante(env: Env, id: String) {
//     //     let mut estudiantes: Map<String, Map<Symbol, String>> = env
//     //         .storage()
//     //         .persistent()
//     //         .get(&symbol_short!("estd"))  // Cambiado a "estd"
//     //         .unwrap_or_else(|| Map::new(&env));

//     //     estudiantes.remove(id.clone());
//     //     env.storage().persistent().set(&symbol_short!("estd"), &estudiantes);  // Cambiado a "estd"
//     // }

//     // pub fn get_estudiante(env: Env, id: String) -> Option<Map<Symbol, String>> {
//     //     let estudiantes: Map<String, Map<Symbol, String>> = env
//     //         .storage()
//     //         .persistent()
//     //         .get(&symbol_short!("estd"))  // Cambiado a "estd"
//     //         .unwrap_or_else(|| Map::new(&env));

//     //     estudiantes.get(id)
//     // }

//     // pub fn buscar_estudiantes(env: Env, nombre: String) -> Option<Map<Symbol, String>> {
//     //     let estudiantes: Map<String, Map<Symbol, String>> = env
//     //         .storage()
//     //         .persistent()
//     //         .get(&symbol_short!("estd"))  // Cambiado a "estd"
//     //         .unwrap_or_else(|| Map::new(&env));

//     //     estudiantes.iter().find(|(_, data)| {
//     //         data.get(symbol_short!("nombre"))
//     //             .map(|n| n == nombre) // Comparar String directamente
//     //             .unwrap_or(false)
//     //     }).map(|(_, data)| data.clone())
//     // }

//    // Documento
// //    pub fn cargar_documento(env: Env, id: String, titulo: String, contenido: String) {
// //     let mut documentos: Map<String, Map<Symbol, String>> = env
// //         .storage()
// //         .persistent()
// //         .get(&symbol_short!("docs"))
// //         .unwrap_or_else(|| Map::new(&env));

// //     let documento_data = map![
// //         &env,
// //         (symbol_short!("titulo"), titulo),
// //         (symbol_short!("contenido"), contenido),
// //     ];

// //     documentos.set(id.clone(), documento_data);
// //     env.storage().persistent().set(&symbol_short!("docs"), &documentos);
// // }

// // pub fn eliminar_documento(env: Env, id: String) {
// //     let mut documentos: Map<String, Map<Symbol, String>> = env
// //         .storage()
// //         .persistent()
// //         .get(&symbol_short!("docs"))
// //         .unwrap_or_else(|| Map::new(&env));

// //     documentos.remove(id.clone());
// //     env.storage().persistent().set(&symbol_short!("docs"), &documentos);
// // }

// // pub fn actualizar_documento(env: Env, id: String, nuevo_titulo: String, nuevo_contenido: String) {
// //     let mut documentos: Map<String, Map<Symbol, String>> = env
// //         .storage()
// //         .persistent()
// //         .get(&symbol_short!("docs"))
// //         .unwrap_or_else(|| Map::new(&env));

// //     if let Some(mut documento_data) = documentos.get(id.clone()) {
// //         documento_data.set(symbol_short!("titulo"), nuevo_titulo);
// //         documento_data.set(symbol_short!("contenido"), nuevo_contenido);
// //         documentos.set(id.clone(), documento_data);
// //         env.storage().persistent().set(&symbol_short!("docs"), &documentos);
// //     }
// // }

// // pub fn adquirir_documento(env: Env, id: String) -> Option<Map<Symbol, String>> {
// //     let documentos: Map<String, Map<Symbol, String>> = env
// //         .storage()
// //         .persistent()
// //         .get(&symbol_short!("docs"))
// //         .unwrap_or_else(|| Map::new(&env));

// //     documentos.get(id)
// // }

// // pub fn buscar_documento(env: Env, titulo: String) -> Option<Map<Symbol, String>> {
// //     let documentos: Map<String, Map<Symbol, String>> = env
// //         .storage()
// //         .persistent()
// //         .get(&symbol_short!("docs"))
// //         .unwrap_or_else(|| Map::new(&env));

// //     documentos.iter().find(|(_, data)| {
// //         data.get(symbol_short!("titulo"))
// //             .map(|t| t == titulo)
// //             .unwrap_or(false)
// //     }).map(|(_, data)| data.clone())
// // }
 

// //Materias
// pub fn registrar_materia(env: Env, id: String, nombre: String, creditos: String) {
//     let mut materias: Map<String, Map<Symbol, String>> = env
//         .storage()
//         .persistent()
//         .get(&symbol_short!("mats"))
//         .unwrap_or_else(|| Map::new(&env));

//     let materia_data = map![
//         &env,
//         (symbol_short!("nombre"), nombre),
//         (symbol_short!("creditos"), creditos),
//     ];

//     materias.set(id.clone(), materia_data);
//     env.storage().persistent().set(&symbol_short!("mats"), &materias);
// }

// pub fn eliminar_materia(env: Env, id: String) {
//     let mut materias: Map<String, Map<Symbol, String>> = env
//         .storage()
//         .persistent()
//         .get(&symbol_short!("mats"))
//         .unwrap_or_else(|| Map::new(&env));

//     materias.remove(id.clone());
//     env.storage().persistent().set(&symbol_short!("mats"), &materias);
// }

// pub fn actualizar_materia(env: Env, id: String, nuevo_nombre: String, nuevos_creditos: String) {
//     let mut materias: Map<String, Map<Symbol, String>> = env
//         .storage()
//         .persistent()
//         .get(&symbol_short!("mats"))
//         .unwrap_or_else(|| Map::new(&env));

//     if let Some(mut materia_data) = materias.get(id.clone()) {
//         materia_data.set(symbol_short!("nombre"), nuevo_nombre);
//         materia_data.set(symbol_short!("creditos"), nuevos_creditos);
//         materias.set(id.clone(), materia_data);
//         env.storage().persistent().set(&symbol_short!("mats"), &materias);
//     }
// }

// pub fn buscar_materia(env: Env, nombre: String) -> Option<Map<Symbol, String>> {
//     let materias: Map<String, Map<Symbol, String>> = env
//         .storage()
//         .persistent()
//         .get(&symbol_short!("mats"))
//         .unwrap_or_else(|| Map::new(&env));

//     materias.iter().find(|(_, data)| {
//         data.get(symbol_short!("nombre"))
//             .map(|n| n == nombre)
//             .unwrap_or(false)
//     }).map(|(_, data)| data.clone())
// }



// }


// mod test;
>>>>>>> 2d5ee31760d78f180066bf4018b6ab05477e0059
