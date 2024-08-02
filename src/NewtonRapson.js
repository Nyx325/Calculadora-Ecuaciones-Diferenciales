const { invoke } = window.__TAURI__.tauri;

window.calcularButton.addEventListener('click', async () => {
  const func = window.funcLbl.value;
  const x0 = window.xLbl.value;
  const derivada = window.derivInput.value;
  const err_limit = window.errLimitInput.value;

  if (!func || !x0 || !derivada || !err_limit) {
    window.error.textContent = 'Todos los campos son obligatorios.';
    return;
  }

  try {
        //Bug raro que me solicita si o si un atributo xFinal para la funcion que sinceramente
        //no tengo idea de dónde sale, pero si no no jala xd
    const result = await invoke('calc_newton', { func, derivada, x0, err_limit, errLimit: err_limit });
    const json = JSON.parse(result);
    
    
    // Limpiar resultados anteriores
    window.error.textContent = '';
    window.newtonOutput.innerHTML = '';
    window.newtonOutput.innerHTML = generateTable(json.euler, ["x", "err"], "Método Newton-Rapson");

  } catch (error) {
    console.error('Error al invocar el comando:', error);
    window.error.textContent = `Error: ${error.message}`;
  }
});

function generateTable(data, headers, title){
  let table = `<h3>${title}</h3><table border="1" cellpadding="5" cellspacing="0"><thead><tr>`;
  headers.forEach(header => {
    table += `<th>${header}</th>`;
  });
  table += `</tr></thead><tbody>`;

  data.forEach(item => {
    table += `<tr>`;
    headers.forEach(header => {
      table += `<td>${item[header]}</td>`;
    });
    table += `</tr>`;
  });

  table += `</tbody></table>`;
  return table;
}