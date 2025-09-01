# TAREAS PARA EL DÍA 2:

### Infra & AWS:
* Crear Key Pair para acceso seguro.
* Configurar un SG con puertos 22 (restringido a tu IP), 80 y 443.
* Activar un Budget Alert en $5.

### Repositorio GitHub:
* Importar el repositorio "starter".
* Configurar protecciòn de rama main (Requerir 1 revisión y CI exitoso).
* Añadir usuario GitHub al archivo CODEOWNERS.

### Servidor Base:
* Crear una instancia EC2 t2.micro con Ubuntu 22.04.
* Verificar conexión por SSH con tu llave.
* Instalar Nginx y activar UFW.

### Colaboración:
* Crear 2 issues en el repositorio.
* Configurar un GitHub Project para organizar el trabajo.

### Criterio de Aceptación Final:
* Tener un servidor respondiendo 200 OK en HTTP, accesible solo por SSH con llave, y el repositorio correctamente configurado.

## INICIO RÁPIDO

1. **Clonar el repositorio**

        git clone https://github.com/SOLx-AR/infra-made-easy.git
        cd infra-made-easy

2. **Agregar tu usuario**

        mkdir users/tu-nombre
        cp ~/.ssh/id_rsa.pub users/tu-nombre/

3. **ETAPA 1 - Deploy tu primer servidor**

        cd ansible
        ansible-playbook playbooks/etapa1-webserver-basico.yml
    
    **¿Qué vas a lograr?**
    * Una instancia EC2 funcionando en AWS
    * Servidor web Nginx configurado
    * Página web personalizada con tu nombre
    * Acceso SSH configurado correctamente
    
    **Pasos:**
    * Configurar AWS: Tener credenciales y EC2 listo
    * Ejecutar playbook: ansible-playbook playbooks/etapa1-webserver-basico.yml
    * Verificar: Abrir tu IP en el navegador
    * ¡Celebrar! 🎉 Ya tienes tu primer servidor automatizado
