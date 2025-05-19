import json
from py_ecc.bls12_381.bls12_381_curve import FQ, FQ2
from py_ecc.bls.point_compression import (
    compress_G1,
    compress_G2
    )
from eth_utils import encode_hex

def g1_from_dec(coords):
    x, y = map(int, coords)
    return (FQ(x), FQ(y))

def g2_from_dec(coords):
    (x0, x1), (y0, y1) = coords
    return (FQ2([int(x0), int(x1)]), FQ2([int(y0), int(y1)]))

# Cargar el verification_key.json exportado por snarkjs
with open("verification_key.json") as f:
    vk = json.load(f)

# Procesar cada parte
alpha_g1 = g1_from_dec(vk["vk_alpha_1"])
beta_g2 = g2_from_dec(vk["vk_beta_2"])
gamma_g2 = g2_from_dec(vk["vk_gamma_2"])
delta_g2 = g2_from_dec(vk["vk_delta_2"])
ic_points = [g1_from_dec(p) for p in vk["IC"]]

# Comprimir a hexadecimal
compressed_vk = {
    "vk_alpha_1": encode_hex(compress_G1(alpha_g1))[2:],  # sin "0x"
    "vk_beta_2": encode_hex(compress_G2(beta_g2))[2:],
    "vk_gamma_2": encode_hex(compress_G2(gamma_g2))[2:],
    "vk_delta_2": encode_hex(compress_G2(delta_g2))[2:],
    "vkIC": [encode_hex(compress_G1(p))[2:] for p in ic_points]
}

# Imprimir para pegar en Aiken
print('vkAlpha: #"' + compressed_vk["vk_alpha_1"] + '"')
print('vkBeta:  #"' + compressed_vk["vk_beta_2"] + '"')
print('vkGamma: #"' + compressed_vk["vk_gamma_2"] + '"')
print('vkDelta: #"' + compressed_vk["vk_delta_2"] + '"')
print("vkIC: [")
for h in compressed_vk["vkIC"]:
    print(f'  #"{h}",')
print("]")
