# VIRTUAL MACHINE
programar en Solana es como programar en una máquina virtual (VM), ya que los programas se ejecutan en un entorno aislado y seguro dentro de la blockchain, similar a como una aplicación se ejecuta en una VM.
En lugar de que el código del programa (o "contrato inteligente" en otras blockchains) se ejecute directamente en los nodos validadores de Solana, se compila a *bytecode* (código de bajo nivel) que luego es interpretado y ejecutado por el *Solana Sealevel Runtime*. Este runtime actúa análogamente a un "sistema operativo" de la máquina virtual.

### Ventajas:
* **Aislamiento y Seguridad:** El runtime de Solana garantiza que cada programa se ejecute de forma aislada, sin poder interferir con otros programas o con el sistema operativo subyacente del validador. Esto es crucial para la seguridad de la red.
* **Determinismo:** La VM de Solana se asegura de que la ejecución de un programa sea determinista. Esto significa que si se le dan las mismas entradas, siempre producirá la misma salida, sin importar en qué validador se ejecute. Esto es fundamental para lograr consenso en una blockchain.
* **Lenguajes de Programación:** Aunque la ejecución final es en bytecode, los desarrolladores pueden escribir sus programas en lenguajes de alto nivel como Rust o C++. Estos lenguajes se compilan a *eBPF* (extended Berkeley Packet Filter), que es el *bytecode* que la VM de Solana entiende y ejecuta. Solana adoptó el *eBPF* por su seguridad, eficiencia y la capacidad de ejecutar programas de forma determinista y aislada.
* **Abstracción:** El modelo de VM abstrae a los desarrolladores de las complejidades de la red subyacente, permitiéndoles centrarse en la lógica del programa en sí.

## SOLANA VIRTUAL MACHINE (SVM):
La SVM (Máquina Virtual de Solana) es el entorno de ejecución de la blockchain de Solana, diseñado para procesar transacciones y contratos inteligentes de forma extremadamente rápida y paralela, usando su tecnología única *Sealevel* para escalar de manera eficiente y soportar miles de transacciones por segundo. A diferencia de la Máquina Virtual de Ethereum (EVM), la SVM aprovecha el procesamiento paralelo para ofrecer una mayor velocidad y menores costos, lo que la convierte en una infraestructura clave para aplicaciones descentralizadas (dApps) en Solana.
 
* **Ejecución de Contratos Inteligentes:** La SVM es el "procesador" que ejecuta los contratos inteligentes (programas en Solana) y maneja las transacciones en la red. 
* **Procesamiento Paralelo:** Puede procesar múltiples transacciones a la vez a diferencia, por ejemplo, de la EVM (Ethereum Virtual Machine) que procesa transacciones secuencialmente,
* **Alta Velocidad y Escalabilidad:** Gracias al procesamiento paralelo, la SVM permite a la blockchain de Solana alcanzar un alto rendimiento, con miles de transacciones por segundo y baja latencia. 
* **Menos Costos:** Su arquitectura está optimizada para la velocidad y el bajo costo, lo que la hace muy eficiente. 
* **Habilitador de dApps:** Permite a los desarrolladores construir y ejecutar aplicaciones descentralizadas (dApps) de manera eficiente en la blockchain de Solana. 

## RUST Y SVM:
El código escrito en Rust se compila y ejecuta en el *Solana Sealevel Runtime*, que cumple una función análoga a un sistema operativo en la *Solana Virtual Machine*. Este proceso se basa en el uso de un objetivo de compilación intermedio llamado eBPF. 

### Proceso de compilación:
* **Código Fuente en Rust:** El desarrollador escriben el código de su programa de Solana (*smart contract*) en el lenguaje de programación Rust.Siendo ideal por su enfoque en la seguridad, el rendimiento y la ausencia del *garbage collector*, lo que es crucial en un entorno de blockchain
* **Compilación a eBPF:** El compilador de Rust, utilizando una herramienta especial llamada *cargo-build-bpf*, no compila el código a código de máquina x86 o ARM tradicional, sino que usa un backend de compilación específico para generar código de bytecode eBPF.
* **Ensamblaje y Vinculación:** El código de bytecode eBPF generado se empaqueta en un archivo binario *.so* (shared object). Este archivo es el programa de Solana que se subirá a la blockchain.
* **Despliegue en la Blockchain:** El archivo .so se envía a la red de Solana a través de una transacción de despliegue. Una vez en la blockchain, el programa está listo para ser invocado.

### Proceso de ejecución en el Solana Sealevel Runtime:
* **Envío de la Transacción:** Un usuario o una aplicación envía una transacción que invoca al programa de Solana. Esta transacción especifica el programa a llamar y los datos de entrada (instrucciones).
* **Carga del Programa:** El validador de Solana que recibe la transacción carga el binario .so del programa eBPF en su memoria.
* **Ejecución en la SVM:** El *Solana Sealevel Runtime* interpreta y ejecuta este bytecode. Algunas veces, *Solana JIT-compiler* (Just-In-Time compiler) compila ese bytecode a código nativo para mayor velocidad.