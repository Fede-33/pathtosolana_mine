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

min 43









