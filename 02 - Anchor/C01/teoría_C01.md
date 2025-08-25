# ACCOUNTS
En Solana todo es una cuenta, con las siguientes características: 
* Se identifica por una dirección única de 256 bits.
* Debe contener algún balance de SOL para existir.
* Puede contener cualquier tipo de dato.
* El almacenamiento de datos se paga con **Rent**.
* Cualquiera puede acreditar SOL o leer sus datos.
* Solo los propietarios *(owners)* pueden debitar o modificar sus datos.

### Estructura:
Las cuentas tienen los siguientes componentes:
* **key:** La dirección de la cuenta.
* **lamports:** Cantidad de unidades mínimas de SOL que contiene. **Lamport = 10^-9 SOL**
* **data:** Información almacenada en forma de arrays de 8 bits (binario).
* **is_executable:** Solo son ejecutables los programas *(Smart Contracts)*.
* **owner:** referencia a la cuenta que es propietaria.

        {
            key: number,
            lamports: number,
            data: Uint8Array,
            is_excecutable: boolean,
            owner: PublicKey
        }

Todos estos campos son modificables, excepto la **key**

## PROGRAMS (SMART CONTRACTS):
Son un tipo especial de cuenta con la flag **is_excecutable** seteada a *True*. La **data** que almacenan está en el formato *dBPF bytecode*, aunque pueden haber sido escritos originalmente en Rust, C/C++, Python, Assembly, Typescript (Poseidon). Los programas son *stateless* (sin estado), no almacenan datos de forma estable dentro de ellos mismos, tan solo la lógica del código para leer o modificar los datos de otras cuentas. Esta separación de lógica y datos permite que los Smart Contracts en Solana puedan ejecutarse de forma paralela. Para que una cuenta invoque a un programa y le sea permitido cambiar la información de otra cuenta, la cuenta que invoca debe ser *owner* de la cuenta a modificar. Finalmente, los programas además de procesar **instrucciones**, pueden enviarlas a otros programas, son componibles entre sí.

### Instructions:
Su esctructura se compone de los siguientes campos:
* **program_id:** La identificación del programa al que se está llamando.
* **keys:** Una lista de cuentas, cada una con una estructura de arreglo, que contenie:
    - **key:** Clave pública para identificarla.
    - **is_mutable:** Si es o no mutable.
    - **is_signer:** Si es o no firmante, es decir, si el validador tiene que verificar la transacción criptográficamente. 
* **data:** Un arreglo de binarios con la información.

        {
            program_id: number,
            keys: Array<{
                key: PubliKey,
                is_mutable: boolean,
                is_signer: boolean,
            }>,
            data: Uint8Array,
        }

La verificación **is_mutable**, le indica al validador que esa cuenta puede ser sometida a una sola instrucción por vez. Es decir, que debe finalizar una instrucción sobre esa cuenta antes de comenzar la siguiente.
En cuanto a la verificación **is_signer** indica que esa cuenta debe proporcionar una firma para la validación, que es la forma principal de verificar la propiedad y autorizar cambios.

### Transactions:
Su esctructura se compone de:
* **message:** Con todos los datos necesarios para la transacción:
    - **instructions:** un arreglo de *instructions*, para que la transacción sea exitosa, todas las instrucciones deben completarse. Si alguna falla, no se realiza ningún cambio.
    - **recent_blockhash:** Referencia al último bloque exitoso, para evitar duplicaciones y mantener una historia de bloques.
    - **fee_payer:** La identificación de la cuenta que paga las tarifas por transacción.
* **signers:** Array con los firmantes.

        {
            message: {
                instructions: Array<Instruction>,
                recent_blockhash: number,
                fee_payer: PublicKey,
                ... }
            signers: Array<Uint8Array>,
        }

## ED25519 ELLIPTIC CURVE:
Si bien en *Solana* todo es una cuenta, existen cuentas del sistema, por ejemplo las que contienen una Wallet o un *Smart Contract*. Cumplen con una propiedad matemática, que el valor de su *PublicKey* coincide con una *curva elíptica*. Por cada combinación de 12 palabras posibles *(seeds)*, se deriva una *PubKey* cuyo valor coincide con dicha curva. En cambio, si se crea una cuenta de datos desde un *Smart Contract*, a su *Key* se le suma un *bump*, para garantizar que no esté dentro de esa curva elíptica. Esto es relevante, porque si la cuenta está dentro de la curva, el firmante es el *Pubkey* de la cuenta. En cambio, si se está fuera de la curva, se considera una cuenta **PDA** (Program Derived Account), en la que el que firma es el **Smart Contract** que creó la *PDA* utilizando las **seeds**. Ejemplo:

Se requiere construir una aplicación de votación en Solana. El programa *(Smart Contract)* necesita una cuenta *PDA* para almacenar el total de votos.

### Cuentas:
* **Cuentas de Votantes:** Cada usuario que quiera votar tendrá una cuenta de wallet normal. La clave pública de esta cuenta está dentro de la curva elíptica ED25519. El usuario tiene la clave privada, y para votar, debe firmar la transacción con ella. Esto prueba su identidad y le autoriza a enviar la instrucción al programa.
* **Cuenta de Votos:** El programa necesita una cuenta para almacenar el conteo total de votos. El desarrollador del programa decide que esta cuenta se generará con las seeds (datos) "votos" + "opción elegida". El programa intentará crear una *PDA* con estas seeds, y el sistema le devolverá una dirección fuera de la curva elíptica y un bump (255). La cuenta de votos resultante es una *PDA* cuya dirección no tiene clave privada.

### Proceso de Votación:
Un usuario envía una transacción al programa de votación. La transacción incluye tres cuentas:
* Su cuenta de wallet (marcada como is_signer: true).
* La cuenta de votos (PDA).
* La llamada al programa de votación (program_id).

**Instrucciones:**
1. El usuario firma la transacción con su clave privada, lo que valida su identidad.
2. El tiempo de ejecución de Solana recibe la transacción y verifica la firma del usuario.
3. El programa de votación se ejecuta. Cuando el programa necesita modificar la cuenta de votos, utiliza las seeds "votos" y "opción elegida", y el *bump* 255 para **"firmar"** la operación. 
4. El tiempo de ejecución de Solana reconoce esta firma como válida porque fue generada por el programa autorizado para la *PDA*.

En este ejemplo, la firma de la cuenta de usuario (dentro de la curva) se usa para autorizar el envío de la instrucción, mientras que la autoridad del programa (con las seeds y el bump) se usa para modificar el estado de la PDA (la cuenta de votos). Esta distinción es fundamental para la seguridad y el diseño de aplicaciones en Solana.

# ANCHOR:
Para inicializar un directorio de *anchor* se utiliza el comando **anchor init [nombre del directorio]**. Se crean una serie de subdirectorios, como *app* donde se supone que se crearía nuestro programa, y *programs* donde se crea otro subdirectorio con el nombre asignado en el *init*, con otro subdirectorio *src* con el archivo *lib.rs* en el que pueden definirse los módulos e instrucciones.

El programa que se incluye en el archivo *lib.rs* tan solo devuelve un mensaje. Para crear una instancia de testeo de un programa se utiliza el comando **anchor test** que combina un **build**, **deploy** y ejecuta los **tests** que se definen en el archivo **tests/counter.ts**. Este comando intenta validar la conexión automáticamente, por lo que se le debe agregar las flags **--skip-local-validator**

# SOLANA:
Para programar en Solana se puede estar conectado a distintas redes, como **mainnet**, **devnet**, **testnet** o emuladores locales **localnet**. Al ejecutar el comando **solana config get** puede observarse la red en el apartado **RPC URL:**

### Configurar:
Para editar las configuraciones del proyecto, se debe editar el archivo **anchor.toml** En este momento debe definirse una wallet para la cuenta de usuario. De lo contrario, al querer interactuar con la *blockchain* retornará un error de **aidrop failed**. Para crear una nueva cuenta de usuario, se ejecuta el comando **solana-keygen new -o [path]** especificando la dirección en la que se debe crear la wallet. Entonces indicará una *pubkey* y las 12 palabras del *seed* Por ejemplo:

    $ solana-keygen new -o ~/.config/solana/id.json
    Generating a new keypair

    For added security, enter a BIP39 passphrase

    NOTE! This passphrase improves security of the recovery seed phrase NOT the
    keypair file itself, which is stored as insecure plain text

    BIP39 Passphrase (empty for none): 

    Wrote new keypair to /home/federinux/.config/solana/id.json
    ===================================================================================
    pubkey: 8ZNFFHh6QcSp44FU5J4rALKnQ56FGdBioBgvZs2V39q9
    ===================================================================================
    Save this seed phrase and your BIP39 passphrase to recover your new keypair:
    door people enlist exotic marriage trophy nation brand unable addict citizen salmon
    ===================================================================================

Para interactuar con la red Solana, la wallet debe tener un balance de SOL. A los efectos de programar de forma principiante, se iniciará un validador local mediante el comando **solana-test-validator**:

    $ solana-test-validator
    Ledger location: test-ledger
    Log: test-ledger/validator.log
    ⠒ Initializing...                                                                                  Waiting for fees to stabilize 1...
    Identity: 8wCX4Tu41gQpQdveh1shU6DgCgEYNFBukBry34XiJNN3
    Genesis Hash: 93Cmqzq2BFCvX2yc4nyCdqyjbQEUx9tJogYaHHq1if6M
    Version: 2.2.20
    Shred Version: 32340
    Gossip Address: 127.0.0.1:1024
    TPU Address: 127.0.0.1:1027
    JSON RPC URL: http://127.0.0.1:8899
    WebSocket PubSub URL: ws://127.0.0.1:8900
    ⠂ 00:00:40 | Processed Slot: 97 | Confirmed Slot: 97 | Finalized Slot: 66 | Full Snapshot Slot: - | I                                                                        

La consola en la que se ejecuta el validador queda ocupada, pero a través de otras consolas podemos hacer deploys, tests, etc. Continuando con el ejemplo, se puede visualizar que, en el archivo *anchor.toml* se encuentran las configuraciones para utilizar el validador de localnet:

    [programs.localnet]
    counter = "CKJdBT3qj1iNkYe8x4Lve3BDg2qEMHVnmkCVfybQfy41"

    [provider]
    cluster = "Localnet"
    wallet = "~/.config/solana/id.json"

Por lo que se debe cambiar la dirección para interactuar a *localhost* mediante el comando **solana config set --url localhost**: 

    $ solana config set --url localhost
    Config File: /home/federinux/.config/solana/cli/config.yml
    RPC URL: http://localhost:8899 
    WebSocket URL: ws://localhost:8900/ (computed)
    Keypair Path: /home/federinux/.config/solana/id.json 
    Commitment: confirmed 

Ahora que se está conectado a un entorno local, el usuario puede acreditarse SOL mediante el comando **solana airdrop [valor] [path]** incluyendo la cantidad que quiera, para hacer los deploys y test que sean necesarios. La dirección del *path* es opcional, tan solo se usa para cuando se quiere agregar SOL a una wallet en particular, sino tomará la del proyecto por defecto:

    $ solana airdrop 100
    Requesting airdrop of 100 SOL

    Signature: 2g34e96BJfYALe4khamLVhHeBExPJhCZE1FxyeSJ4rKa4Ljg8nLoUrLJoSYt3K1VTUCdsMMgjd3WQACX4VWZYTko

    500000100 SOL

Para verificar la cantidad de SOL que contiene la wallet, se utiliza el comando **solana balance [path]** siendo la ruta también opcional.