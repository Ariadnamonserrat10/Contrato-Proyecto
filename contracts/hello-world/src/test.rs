#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, symbol_short};

#[test]
fn test_estudiante_operations() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Insertar estudiante
    client.insert_estudiante(
        &String::from_str(&env, "1"),
        &String::from_str(&env, "Carlos"),
        &String::from_str(&env, "carlos@example.com"),
    );

    // Verificar inserción
    let estudiante = client.get_estudiante(&String::from_str(&env, "1"));
    assert!(estudiante.is_some());
    let estudiante_data = estudiante.unwrap();
    assert_eq!(
        estudiante_data.get(symbol_short!("nombre")),
        Some(String::from_str(&env, "Carlos"))
    );
    assert_eq!(
        estudiante_data.get(symbol_short!("email")),
        Some(String::from_str(&env, "carlos@example.com"))
    );

    // Editar estudiante
    client.edit_estudiante(
        &String::from_str(&env, "1"),
        &String::from_str(&env, "Carlos Gómez"),
        &String::from_str(&env, "carlos.gomez@example.com"),
    );

    // Verificar edición
    let updated_estudiante = client.get_estudiante(&String::from_str(&env, "1"));
    assert!(updated_estudiante.is_some());
    let updated_data = updated_estudiante.unwrap();
    assert_eq!(
        updated_data.get(symbol_short!("nombre")),
        Some(String::from_str(&env, "Carlos Gómez"))
    );
    assert_eq!(
        updated_data.get(symbol_short!("email")),
        Some(String::from_str(&env, "carlos.gomez@example.com"))
    );

    // Buscar estudiante
    let search_result = client.buscar_estudiantes(&String::from_str(&env, "Carlos Gómez"));
    assert!(search_result.is_some());

    // Eliminar estudiante
    client.delete_estudiante(&String::from_str(&env, "1"));

    // Verificar eliminación
    let deleted_estudiante = client.get_estudiante(&String::from_str(&env, "1"));
    assert!(deleted_estudiante.is_none());
}
