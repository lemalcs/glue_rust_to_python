from glue_rust_to_python import validate_modulus_10

def test_modulus_10_compliant_number():
    result=validate_modulus_10("16467821")

    assert result

def test_not_modulus_10_compliant_number():
    result=validate_modulus_10("2016811009")

    assert result==False