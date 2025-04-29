#![no_std]

use soroban_sdk::{contract, contractimpl, map, Env, String, Map, Symbol, symbol_short};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn insert_estudiante(env: Env, id: String, nombre: String, email: String) {
        let mut estudiantes: Map<String, Map<Symbol, String>> = env
            .storage()
            .persistent()
            .get(&symbol_short!("estd"))  // Cambiado a "estd"
            .unwrap_or_else(|| Map::new(&env));

        let estudiante_data = map![
            &env,
            (symbol_short!("nombre"), nombre),
            (symbol_short!("email"), email),
        ];

        estudiantes.set(id.clone(), estudiante_data);
        env.storage().persistent().set(&symbol_short!("estd"), &estudiantes);  // Cambiado a "estd"
    }

    pub fn edit_estudiante(env: Env, id: String, new_nombre: String, new_email: String) {
        let mut estudiantes: Map<String, Map<Symbol, String>> = env
            .storage()
            .persistent()
            .get(&symbol_short!("estd"))  // Cambiado a "estd"
            .unwrap_or_else(|| Map::new(&env));

        if let Some(mut estudiante_data) = estudiantes.get(id.clone()) {
            estudiante_data.set(symbol_short!("nombre"), new_nombre);
            estudiante_data.set(symbol_short!("email"), new_email);
            estudiantes.set(id.clone(), estudiante_data);
            env.storage().persistent().set(&symbol_short!("estd"), &estudiantes);  // Cambiado a "estd"
        }
    }

    pub fn delete_estudiante(env: Env, id: String) {
        let mut estudiantes: Map<String, Map<Symbol, String>> = env
            .storage()
            .persistent()
            .get(&symbol_short!("estd"))  // Cambiado a "estd"
            .unwrap_or_else(|| Map::new(&env));

        estudiantes.remove(id.clone());
        env.storage().persistent().set(&symbol_short!("estd"), &estudiantes);  // Cambiado a "estd"
    }

    pub fn get_estudiante(env: Env, id: String) -> Option<Map<Symbol, String>> {
        let estudiantes: Map<String, Map<Symbol, String>> = env
            .storage()
            .persistent()
            .get(&symbol_short!("estd"))  // Cambiado a "estd"
            .unwrap_or_else(|| Map::new(&env));

        estudiantes.get(id)
    }

    pub fn buscar_estudiantes(env: Env, nombre: String) -> Option<Map<Symbol, String>> {
        let estudiantes: Map<String, Map<Symbol, String>> = env
            .storage()
            .persistent()
            .get(&symbol_short!("estd"))  // Cambiado a "estd"
            .unwrap_or_else(|| Map::new(&env));

        estudiantes.iter().find(|(_, data)| {
            data.get(symbol_short!("nombre"))
                .map(|n| n == nombre) // Comparar String directamente
                .unwrap_or(false)
        }).map(|(_, data)| data.clone())
    }


   //funcione4s del docente 
   
   
}

mod test;


