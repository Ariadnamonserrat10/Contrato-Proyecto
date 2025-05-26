// Variables del contrato
const CONTRACT_ID = 'CDS3VGIAZFIZUG3GL6LWAXLOXWCRIIBDPPRS4225LLLEHPGTFDNF4PQK'; // <- reemplazar
const RPC_URL = 'https://rpc-futurenet.stellar.org'; // o local

// Elementos del DOM
const form = document.getElementById('userForm');
const nameEl = document.getElementById('name');
const emailEl = document.getElementById('email');
const passwordEl = document.getElementById('password');
const result = document.getElementById('result');

// Botones adicionales
document.getElementById('editBtn').onclick = () => ejecutarAccion('edit_user');
document.getElementById('deleteBtn').onclick = () => ejecutarAccion('delete_user');
document.getElementById('getBtn').onclick = () => ejecutarAccion('get_user');

// Insertar usuario al enviar el formulario
form.addEventListener('submit', (e) => {
  e.preventDefault();
  ejecutarAccion('insert_user');
});

async function ejecutarAccion(accion) {
  const name = nameEl.value.trim();
  const email = emailEl.value.trim();
  const password = passwordEl.value.trim();

  if (!name) {
    alert('El nombre es obligatorio');
    return;
  }

  result.textContent = `Ejecutando acción: ${accion}...`;

  // Simulación de integración con contrato (soroban-client o fetch a backend)
  try {
    // 👉 Aquí deberías usar soroban-client o soroban.js
    // Este bloque es solo demostrativo
    result.textContent = `→ Acción: ${accion}\n→ Nombre: ${name}\n→ Email: ${email}\n→ Password: ${password}`;

    // Ejemplo pseudocódigo con soroban-client:
    // const tx = await server.invoke({ contractId: ..., method: accion, args: [...] });

    // Simulación de respuesta
    if (accion === 'get_user') {
      result.textContent = `Usuario encontrado:\nNombre: ${name}\nEmail: demo@correo.com\nContraseña: *****`;
    }

  } catch (err) {
    console.error(err);
    result.textContent = `❌ Error: ${err.message || err}`;
  }
}
