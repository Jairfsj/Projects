function sendMessage() {
  const message = document.getElementByid("message").value;

  fetch('/api/chat', {//Rota para o backend
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({message}),

  })
  .then(response => response.json())
  .then(data => {
    console.log('Mensagem enviada:', data);
    //Aqui mensagem para interface do chat
  })
  .catch((error) => {
    console.error('Erro:', error);
  });
}
