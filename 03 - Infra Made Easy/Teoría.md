# INFRAESTRUCTURA COMO CÓDIGO
Infraestructura como Código (IaC) es el proceso de gestionar y aprovisionar recursos de infraestructura de TI, como redes, máquinas virtuales, balanceadores de carga y bases de datos, utilizando archivos de configuración legibles por máquina, en lugar de hacerlo de forma manual. En esencia, IaC aplica los principios y prácticas del desarrollo de software, como el control de versiones, las pruebas y la automatización, a la gestión de la infraestructura.

### Beneficios:

* **Reducción de error humano:** Un error muy común es el *Configuration Drift*, cuando se implementan nuevas características pero no se actualiza la documentación, resultando en que la infraestructura que se describe, no es la que en realidad existe. A su vez, evita los *pet servers*, infrestructuras que dependen del conocimiento y experiencia de uno o un reducido grupo de técnicos, lo que a largo plazo implica complicaciones para actualización o respuesta ante errores.
* **Repetibilidad y escalabilidad:** Al usar el mismo código, puedes replicar un entorno de desarrollo, prueba o producción de manera idéntica. La infraestructura resulta predecible, repetible y escalable.
* **Seguridad:** En la infraestructura manual, la seguridad se implementa como un parche posterior, mientras que en IaC se incluye en el diseño inicial como componente fundamental. 
* **Velocidad y eficiencia:** La automatización reduce significativamente el tiempo de aprovisionamiento de recursos, permitiendo a los equipos de desarrollo y operaciones (DevOps) lanzar nuevos servicios más rápido.
* **Control de versiones:** El código de la infraestructura puede almacenarse en sistemas de control de versiones como Git. Esto permite rastrear cambios, revertir a versiones anteriores y colaborar en la configuración de la infraestructura de manera similar a como se hace con el código de las aplicaciones. Facilidad para auditar modificaciones.
* **Colaborativa:** La infraestructura puede estar diseñada por un grupo de pares, que revisan y aprueban su actualización mediante *pull requests*. 
* **Reducción de costos:** La automatización ayuda a optimizar el uso de recursos y a reducir el tiempo que los ingenieros dedican a tareas manuales.

## ENFOQUES

### Enfoque imperativo: 
Con este enfoque, se definen los pasos específicos para llegar al estado deseado. Es decir, *el cómo*. Se le indica a la herramienta "crear la máquina virtual, luego instala el servidor web y finalmente configurar el firewall". Herramientas como *Puppet* y *Chef* suelen usarse en este modelo. Por ejemplo:

    apt update
    apt install nginx -y
    systemctl start nginx
    systemctl enable nginx

Uno de los problemas que pueden surgir es que no se considera el estado actual. Si Nginx ya está instalado, puede fallar o causar efectos indeseados.


### Enfoque declarativo:
En este método, se describe el estado final deseado de la infraestructura, y la herramienta de IaC se encarga de determinar y ejecutar los pasos necesarios para alcanzar ese estado. Se indicaría "Crear una máquina virtual con 8 GB de RAM y un disco de 100 GB". Herramientas populares como *Terraform*, *Ansible* y *AWS CloudFormation* operan bajo este modelo. Por ejemplo:

    - name: Asegurar que nginx esté instalado
        ansible.builtin.apt:
            name: nginx
            state: present
    - name: Asegurar que nginx esté en ejecución
        ansible.builtin.service:
            name: ngonx
            state: Started
            enabled: yes

Tiene en cuenta la comprobación del estado actual para determinar los pasos necesarios, siendo un enfoque más robusto, performante y seguro.

**CARACTERÍSTICAS:** En enfoque declarativo se dan naturalmente dos propiedades que son piedras angulares de la IaC:

* **Idempotencia:** Es la propiedad de una operación que garantiza que se obtendrá el mismo resultado, sin importar cuántas veces se ejecute. Por ejemplo, una orden para "instalar el servidor web Nginx" es idempotente, ya si Nginx ya está instalado, la herramienta de automatización no hará nada, pero si no lo está, lo instalará. No importa cuantas veces se ejecute, el estado final será el mismo. 
* **Convergencia:** Se refiere al proceso de llevar un sistema desde su estado actual a un estado final deseado y predefinido. Las herramientas de automatización, como Ansible o Terraform, "convergen" el sistema hacia el estado que se describe en su código de configuración. La herramienta compara el estado actual del sistema con el estado deseado y aplica las acciones necesarias para cerrar la brecha. Un beneficio extra es que, si el sistema se desvía del estado deseado, la herramienta lo corrige automáticamente. Por ejemplo, si un usuario detiene Nginx manualmente, la próxima vez que se ejecute Ansible, lo detectará como estado actual y volverá a iniciar el servicio para llegar al estado deseado. 

En conclusión, la *idempotencia* es la propiedad de cada paso individual del proceso, que asegura que cada operación se pueda ejecutar de forma segura y repetida hasta que se logre la *convergencia*.

En el enfoque imperativo, para lograr la idempotencia y convergencia, estas deben ser implementadas específica y manualmente. Un script imperativo debe especificar la lógica para garantizar que cada paso sea idempotente. Por ejemplo, antes de crear un directorio, el script debe verificar si ya existe. Esto requiere más código, verificaciones condicionales y un diseño cuidadoso para evitar errores. De esto resulta que la convergencia no sea automática, ya que se necesita una lógica más compleja para comparar cada posible estado, con el estado objetivo, y decidir qué pasos ejecutar u omitir.

## INMUTABILIDAD
Es la propiedad que se consigue con la culminación del proceso de cambio de paradigma de servidores "mascotas" al de "ganado". Inicialmente, la infraestructura del tipo manual era diseñada individual y específicamente. Y de la misma manera se debía mantenerla, aplicando parches cada vez que se rompía. Tal como una mascota que se vuelve única, fragil e irremplazable. Actualmente se trata de que la infraestructura se conforme por servidores anónimos, idénticos, repetibles y reemplazables o desechables. Si se necesita de una actualización lo suficiéntemente importante, no conviene aplicar parches, sino destruirlo y crear un servidor nuevo, actualizando el código que lo genera. Siendo entonces predecible y evitando el *configuration drift*.

# AWS
**AMAZON WEB SERVERS** Es la plataforma de nube más grande y completa en la actualidad, propiedad de Amazon.com. En lugar de que las empresas compren, mantengan y administren sus propios servidores físicos y centros de datos, **AWS** permite "alquilar" recursos de TI (como poder de cómputo, almacenamiento y bases de datos) a través de Internet, con un modelo de pago por uso. Durante el primer año de uso, se ofrece a los nuevos usuarios USD100 de crédito y algunos servidores gratuitos para ejecutar. Algunos de sus servicios y características son:

* **Amazon EC2 (Elastic Compute Cloud):** Permite ejecutar servidores virtuales, lo que es esencialmente una computadora en la nube que puedes usar para alojar sitios web, ejecutar aplicaciones o realizar cálculos. Cuando se eligen servidores virtuales de Linux, es recomendable seleccionar sistemas con LTS (Long Time Support) que son las distribuciones finalizadas en .04.
* **Security Groups (SG):** es un firewall virtual con estado que actúa como primera línea de defensa para tus recursos, como las instancias EC2, controlando el tráfico entrante y saliente permitido a través de reglas definidas por el usuario, en lugar de denegarlo
* **Key Pair:** El método de acceso seguro mediante SSH, con una clave pública almacenada en AWS y una privada en poder del administrador del servidor.
* **Tags:** Etiquetas para organizar recursos, automatizar procesos y controlar costos. Son fundamentales en las buenas prácticas, para una gestión eficiente.
* **Budget Alerts:** Sistema de protección ante cambios bruscos de facturación de AWS. A medida que aumenta el uso de los servidores, por ejemplo, por tráfico en un sitio web, se activan las alertas de presupuesto, para que el administrador sea conciente de los incrementos de facturación en tiempo real, y no se encuentre con una deuda sorpresa al final del periodo.
* **Identity and Access Managment (IAM):** Gestor de creación de usuarios, de acceso a servicios y recursos, y de administración de permisos. 

# ANSIBLE
Herramienta de automatización de código abierto utilizada para la gestión de la configuración, el despliegue de aplicaciones, la orquestación y el aprovisionamiento. 

### Beneficios:

* **Simplicidad:** Es fácil de aprender y usar, mayormente codificado en YAML, es legible y se parece a instrucciones en un lenguaje natural.
* **Sin agente:** No necesita software adicional instalado en los nodos (máquinas) que gestiona, lo que simplifica enormemente la configuración y reduce la sobrecarga en las máquinas gestionadas.
* **Potencia:** Permite automatizar tareas complejas, desde una simple actualización de software hasta el despliegue completo de un clúster de servidores.
* **Seguridad:** Se comunica con los nodos a través de SSH (Secure Shell) para sistemas Linux/Unix o WinRM (Windows Remote Management) para sistemas Windows.
* **Comunidad y módulos:** Cuenta con una gran comunidad de usuarios y una amplia biblioteca de módulos preexistentes. Estos módulos son fragmentos de código reutilizables para realizar tareas específicas, como gestionar servicios, archivos, paquetes, etc.

## COMPONENTES

### Inventario:
Es una lista de los hosts que se van a gestionar. Puede ser de dos tipos:

* **Estático:** Se definen los webservers y databases mediante direcciones IP, hosts, y permite declarar algún nombre particular (alias), dominio, dirección interna y/o externa, etc. Por ejemplo, definiendo dos grupos *webserver* y *database*, con alias y dirección IP, la cual se asocia a cada alias mediante la variable *ansible_host*: 

        [webserver]
        server_1 ansible_host=18.23.45.67
        server_2 ansible_host=18.23.45.68

        [database]
        db_1 ansible_host=18.23.45.69
        db_2 ansible_host=18.23.45.70
    Este tipo de inventario es simple y rídigo, adecuado para entornos pequeños y estables. Pero, cada vez que se agregue o elimine un servidor se debe actualizar el archivo, lo que da lugar a errores humanos y resulta poco escalable.

* **Dinámico:** Este tipo de inventario se va actualizando de manera dinámica, en función de las etiquetas que se implementan cuando se crea el servicio. No es un archivo fijo, sino un script o programa que que Ansible corre cada vez que necesita la lista de servidores. Este script se conecta a una fuente de datos externa para obtener la información de los hosts, como la dirección IP, el nombre del servidor y sus propiedades (tags). Las **tags** son fundamentales ya que el Script las interpretará para que Ansible cree los grupos de forma automática. Para el caso de configurar un inventario dinámico en **AWS**, que funcione tal como el ejemplo anterior (estático), primeramente se deben crear las instancias *EC2* en la consola de *AWS* y añadir los alias y etiquetas:

        18.23.45.67: Name: server_1 role: webserver
        18.23.45.68: Name: server_2 role: webserver
        18.23.45.69: Name: db_1 role: database
        18.23.45.70: Name: db_2 role: database

    En el mismo directorio donde se encontrarán los *playbooks* se crea el siguiente archivo .yaml:

        plugin: aws_ec2
        regions:
          - us-east-1  
        hostnames:
          - tag:Name
        keyed_groups:
          - key: tags.role
            prefix: ''
    **Plugin** le indica a Ansible que use el plugin de inventario *aws_ec2*. **Regions** especifica la región geográfica donde se encuentran las intancias. **Hostmanes** define qué información de la instancia se usará como *ansible_host*. **Keyed_groups** crea grupos basados en pares clave:valor, donde *key:tags.role* le dice a Ansible que use la etiqueta *role* en las instancias definidas en la consola de AWS, y *prefix:''* señala que el grupo se nombrará directamente con el valor de la instancia.  De esta manera, cada vez que se ejecute Ansible, generará el inventario dinámico en formato JSON, que se vería así:

        {
          "webserver": {
            "hosts": [
              "server_1",
              "server_2"
            ]
          },
          "database": {
            "hosts": [
              "db_1",
              "db_2"
            ]
          },
          "_meta": {
            "hostvars": {
              "server_1": {
                "ansible_host": "18.23.45.67"
              },
              "server_2": {
                "ansible_host": "18.23.45.68"
              },
              "db_1": {
                "ansible_host": "18.23.45.69"
              },
              "db_2": {
                "ansible_host": "28.23.45.70"
              }
            }
          }
        }
    Este inventario dinámico, es equivalente al inventario estático definido anteriormente. Si bien su configuración inicial es más compleja, su principal ventaja es que la escalabilidad, ya que actualizando los datos de la fuente, en este caso AWS, puede mantenerse la infraestructura sin modificar el código archivo .yaml o .yml.

### Playbook:
Archivos escritos en YAML (Yet Another Markup Language), que describen la lógica de automatización, es decir, los pasos o "tareas" que Ansible debe ejecutar en los hosts. La identación es fundamental y obligatoria en YAML para definir la estructura y la jerarquía de los datos. La comunidad de Ansible recomienda usar 2 espacios para cada nivel de anidamiento. Un ejemplo, para automatizar los inventarios definidos anteriormente:

        - name: Configurar webservers
          hosts: webserver
          become: yes
          tasks:
            - name: Instalar Nginx
              ansible.builtin.apt:
              name: nginx
              state: present
              update_cache: yes
        
            - name: Iniciar y habilitar Nginx
              ansible.builtin.service:
              name: nginx
              state: present
              enabled: yes

        - name: Configurar bases de datos
          hosts: database
          tasks:
            - name: Instalar MySQL
              ansible.builtin.apt:
              name: mysql-server
              state: present
              update_cache: yes

            - name: Iniciar y habilitar MySQL
              ansible.builtin.service:
              name: mysql
              state: present
              enabled: yes

La sección **Configurar webservers** está dirigida a todos los hosts del grupo **webserver**, ejecutando las tareas como *sudo* mediante la etiqueta **become: yes** debido a que será necesario para instalar software. la latera **Instalar Nginx** usa el módulo **ansible.builtin.apt** para instalar el paquete del servidor web **Nginx**, especificando el parámetro **update_cache: yes** para verificar que el índice de paquetes esté actualizado antes de la instalación. La tarea **Iniciar y habilitar Nginx** usa el módulo **ansible.builtin.service** para asegurarse de que el servicio de Nginx esté corriendo **state: present** y se inicie automáticamente en el arranque del sistema **enabled: yes**. La sección **Configurar bases de datos** es análoga a **Configurar servers**, pero aplicada a los hosts del grupo **database**. 

### Rol:
Una estructura estandarizada para organizar y empaquetar la automatización (tareas, variables, archivos y plantillas) con un propósito específico, para poder usarlas de forma modular con diferentes playbooks. Por ejemplo, una serie de tareas para configurar un servidor web (instalar Nginx, configurar el firewall y desplegar los archivos, etc) en lugar de escribir todas estas tareas en un solo playbook, pueden agruparse en una estructura de directorio estandarizada, llamada *webserver/*:

    roles/
    └── webserver/
        ├── defaults/       # Variables por defecto
        ├── handlers/       # Tareas que solo se ejecutan cuando son notificadas
        ├── tasks/          # El playbook principal del rol
        ├── templates/      # Archivos de plantilla (ej. configuración de Nginx)
        ├── files/          # Archivos estáticos
        └── vars/           # Variables para el rol

Cada subdirectorio tiene un propósito específico. **tasks/** contiene el archivo main.yml, que define todas las tareas del rol. **templates/** guarda archivos de plantilla que se pueden personalizar con variables. **handlers/** incluye tareas que se ejecutan solo cuando se activan (por ejemplo, reiniciar un servicio). El uso de roles tiene los siguientes beneficios:

* **Reutilización:** Puedes usar el mismo rol en múltiples playbooks para configurar diferentes servidores.
* **Organización:** Mantiene el código limpio, legible y modular. En lugar de tener un playbook extenso,se divide en varios roles pequeños y bien definidos.
* **Compartir:** Los roles son fáciles de compartir con otros miembros del equipo o con la comunidad de Ansible a través de *Ansible Galaxy*, una plataforma que almacena roles preexistentes para casi cualquier propósito.

### Colección:
Un formato de distribución que agrupa roles, módulos y otros plugins rlacionados con un paquete cohesivo. Por ejemplo **amazon.aws** proporciona módulos específicos para interactuar con servicios de AWS, o **community.general** que incluye una amplia variedad de módulos útiles desarrollados por la comunidad Ansible. Todas las colecciones requeridas de un repositorio específico se pueden detallar en un archivo **requirements.yml**:

    ansible-galaxy collection install amazon.aws
    ansible-galaxy collection install comunity.general

Este puede ejecutarse de forma simple mediante el comando **pip install requirements**.

# SEGURIDAD

## BASE 1: ACCESO Y RED
### Capa 1 - AWS Security Group (Perímetro):
Funciona como un "firewall virtual" que controla el tráfico de los servidores. Puede permitir el tráfico exclusivamente en los puertos necesarios: 
    
    - Puerto 22 (SSH): Para la administración remota de servidores.
    - Puerto 80 (HTTP): Para el tráfico de un servidor web no encriptado.
    - Puerto 443 (HTTPS): Para el tráfico de un servidor web encriptado.

  Cómo regla de oro, el puerto 22 solo debe estar restringido una IP propia o de confianza (VPN de oficina). Nunca establecer 0.0.0.0/0 (cualquier dirección IP) para SSH. 
### Capa 2 - UFW en el Host (Firewall interno):
**UFW** significa Uncomplicated Firewaal. Es la capa de defensa en profundidad que garantiza que, si la Capa 1 falla o estuviera mal configurada, el firewall del propio host seguirá activo. es una interfaz simple para gestionar las reglas de **iptables** el firewall de Linux.

        sudo ufw allow 80/tcp 
        sudo ufw allow 443/tcp
        sudo ufw allow from 203.0.113.4 to any port 22
        sudo ufw enable

  Las reglas anteriores permiten el tráfico para los puertos HTTP y HTTPS, y restringe las conexiones SSH solo a la dirección IP 203.0.113.4. Finalmente activa el firewall con las reglas definidas.

### Capa 3 - Acceso solo por SSH:
Deshabilitar completamente la autenticación por contraseña en la configuración SSH, en el archivo **/etc/ssh/sshd_config** establecer:

        PasswordAuthentication no
        CallengeResponseAuthentication no

  Solo quienes posean la clave privada pordán acceder al servidor.

## BASE 2: PRIVILEGIOS Y SECRETOS

### Principio del Menor Privilegio (PoLP):
* **AWS (IAM):** Crear usuarios IAM con permisos mínimos e indispensables para sus tareas específicas. No permitir permisos **sudo** ni utilizar cuenta **root** para operaciones que no lo requieran.
* **Linux:** Nunca permitir el login directo con usuario **root** mediante SSH, para esto se debe configurar en **/etc/ssh/sshd_config**

        PermitRootLogin no
        
    Utilizar siempre un usuario normal para logearse y escalar provilegios con **sudo** cuando sea necesario.

### Gestión de Secretos:
Es necesario gestionar API keys, contraseñas internas de bases de datos y otras credenciales, de forma segura. **Nunca se deben comentar en texto plano de Git**. En cambio se recomienda:
*  **Ansible Vault:** es la solución nativa de Ansible para encriptar archivos de variables, playbooks y cualquier otro tipo de dato. Es una forma sencilla y efectiva de gestionar secretos dentro del propio ecosistema de Ansible (Workflow). 
    - Se encripta un archivo **ansible-vault encrypt secrets.yml**, para que sea ilegible sin la contraseña, y pueda ser guarddo en el repositorio Git. 
    - Al ejecutar el playbook, agregar la opción **ansible-playbook site.yml --ask-vault-pass** para desencriptar el archivo en tiempo de ejecución, sin exponer las credenciales.
*  **Entornos empresariales:** Usar herramientas como **AWS Secrets Manager** o **HashCorp Vault**. Estas herramientas son ideales para arquitecturas de microservicios o entornos que requieren una rotación automática de secretos, una auditoría detallada y un control de acceso estricto.

# COLABORACIÓN PROFESIONAL EN GITHUB

## FLUJO DE TRABAJO
1. Todo trabajo comienza con un **issue** bien documentado que describe el problema a solucionar o la mejora a implementar.
2. Se crea una **Branch** específica para trabajar en ese **issue**, manteniendo la rama **main** protegida.
3. El código se entrega mediante un **Pull Request (PR)** descriptivo.
4. El PR es **revisado** por alguno de los miembros del equipo.
5. Los **chequeos automáticos (CI)** verifican que el código cumple con los estándares.
6. Si todo está en condiciones, se realiza el **Merge** a la rama **main**.

## HERRAMIENTAS
### Issue & PR Templates:
Platillas predefinidas que aseguran que cada issue y PR contangan toda la información necesaria:
* Contexto detallado del problema.
* Pasos para reproducirlo.
* Checklist de *Definition of Done DoD* (Qué es necesario para cunplir con los requisitos del cliente)
* Impacto esperado.

### Codeowners:
Archivo que asigna automáticamente revisores para los PR según los archivos modificados. Especificando los archivos por *path*, y los grupos mediante *@nombre*:

    /ansible/roles/database/ @equipo-db
    /ansible/roles/webserver/ @equipo-ws
    *.yml @equipo-devops

### Labels:
Etiquetas para categorizar y filtrar el trabajo. Por ejemplo:
* **Bug:** Problemas que necesitan corrección.
* **Feature:** Nuevas funcionalidades.
* **Security:** Implementaciones de seguridad.
* **Equipo-a:** Tareas asignadas específicas para un equipo.

### GitHub Proyects:
Herramienta gratuita e integrada a los repositorios. Es un tablero de Kanban para visualizar el flujo de trabajo, que permite optimizarlo en spectos como:

* Detectar cuellos de botella en el proceso.
* Visibilizar el progreso.
* Gestionar prioridades.
* Facilitar la planificación de sprints.

## STARTER
El repositorio de arranque de GitHub. Inicialmente se debe definir una **Estructura**, del tipo:

    /ansible
        /roles
        /inventories
        /playbooks
    /.github
        /workflows
            ansible-lint.yml
            yamllint.yml
        CODEOWNERS
        PULL_REQUEST_TEMPLATE.,d
    README.md

Dentro de los **workflows** gratuitos que ofrece GitHub, están **ansible-lint** que verifica que los playbooks se ajusten a buenas prácticas de Ansible, y **yamlint** que controla la sintaxis e identación de los archivos YAML. Ambos se ejecutan automáticamente en cada PR. Un ejemplo de configuración de **ansible-lint.yml** sería:

    name: Ansible Lint
    
    on:
      pull_request:
      paths:
      - 'ansible/**'
    
    jobs:
      ansible-lint:
      runs-on: ubuntu-latest
      steps:
      - uses: actions/checkout@v3
      - name: Set up Python
      uses: actions/setup-python@v4
      with:
      python-version: '3.10'
      - name: Install dependencies
      run: |
      python -m pip install --upgrade pip
      pip install ansible.lint
      - name: Lint Ansible
      run: |
      ansible-lint ansible/

Crea un sistema automatizado para revisar el código de Ansible en GitHub, para garantizar que los playbooks sigan un estándar de calidad y estén libres de errores antes de ser fusionados en la rama principal: 
name: Ansible Lint: Le da un nombre a este flujo de trabajo para que sea fácil de identificar en GitHub.

* **on:** Define los eventos que activarán el flujo de trabajo.
    - **pull_request:** El flujo de trabajo se ejecutará automáticamente cada vez que alguien inicie un PR.
    - **paths: - 'ansible/\*\*'** Esta condición indica que el flujo de trabajo solo se activará si los archivos modificados en el PR están dentro del directorio **ansible/**. Esto evita que se ejecute en cambios que no están relacionados con el código de Ansible.
* **jobs:** Contiene los trabajos que se ejecutarán en el flujo de trabajo. Aquí solo hay un trabajo.
    - **ansible-lint:** El nombre del trabajo.
    - **runs-on: ubuntu-latest** Le indica a GitHub que el trabajo se ejecutará en un servidor virtual con la última versión de Ubuntu.
    - **steps:** Son las acciones que se realizarán secuencialmente dentro del trabajo.
        - **\- uses: actions/checkout@v3:** Un paso estándar en GitHub Actions que clona el repositorio en el servidor virtual para que el flujo de trabajo pueda acceder al código.
        - **\-name: Set up Python:** Instala una versión específica de Python (3.10), ya que ansible-lint es una herramienta de Python.
        - **\-name: Install dependencies:** Instala las dependencias necesarias. Primero, actualiza **pip** y luego instala la herramienta **ansible-lint**.
        - **\-name: Lint Ansible:** Este es el paso final. Ejecuta el comando **ansible-lint** sobre el directorio **ansible/**. La herramienta escanea todos los archivos dentro de ese directorio y reporta cualquier error de sintaxis, violaciones de estilo o malas prácticas que encuentre.





