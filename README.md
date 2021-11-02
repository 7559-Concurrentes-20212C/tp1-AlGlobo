# Trabajo Práctico 1

## Objetivo

Deberán implementar una aplicación en Rust que modele el sistema de reservas de AlGlobo.com. Los pedidos de reserva se leerán desde un archivo y el webservice de cada aerolínea se simulará con sleeps aleatorios, y su resultado utilizando también variables aleatorias (random).

## Desarollo

Ejecución con cargo

```bash
cargo run
```
## Configurables
El programa toma los datos de los siguientes archivos configurables:
`reservations`: pedido de reserva ingresa al sistema e indica aeropuerto de origen, de destino, aerolínea preferida y si la reserva es por paquete o solo vuelo.
`valid_airlines`: cada linea describe aerolinea, probabilidad de aceptar request, tiempo de espera en un caso de reintento por rechazo de request.

## Output
El programa devuelve el log de operaciones por stdout y por el archivo `stats_results`.