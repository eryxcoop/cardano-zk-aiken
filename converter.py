def decimal_strings_to_hex(data):
    """
    Convierte una lista de listas de strings decimales a un string hexadecimal.
    
    Args:
        data (list[list[str]]): Lista de listas con strings que representan decimales.

    Returns:
        str: Representación hexadecimal del número construido al concatenar todos los strings.
    """
    # Aplanar la lista de listas en una sola lista
    flat_list = [item for sublist in data for item in sublist]

    # Concatenar los strings como si fueran dígitos
    decimal_string = ''.join(flat_list)

    # Convertir el string a entero decimal
    decimal_number = int(decimal_string)

    # Convertir a hexadecimal
    hex_string = hex(decimal_number)

    return hex_string

# Ejemplo de uso
lista = [
  [
  "906632857760023531645503951339882088205787817173276500915769102387296842114679272227848424151070031898469938058437",
  "1904188708159344054082931783797378035061162915446794498830735860801564016938807696363319341958139678318666238701243",
  "1"
 ]
 ]
resultado = decimal_strings_to_hex(lista)
print(resultado)  # ejemplo: '0x1234567890'
