use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockDecryptMut, KeyIvInit};
use aes::Aes256;
use cbc::Decryptor;
use reqwest::blocking::Client;

fn connect(client: &Client, uuid: &str) -> String {
    let cookie = format!("everybody-codes={}", uuid);
    let user_data = client.get("https://everybody.codes/api/user/me")
        .header("Cookie", cookie)
        .send()
        .expect("Connection (seed) failed")
        .json::<serde_json::Value>()
        .expect("Deserialization (seed) failed");

    user_data.get("seed")
        .expect(&format!("Seed does not exist in response {}", user_data))
        .as_number()
        .expect(&format!("Seed was not a number {}", user_data))
        .to_string()
}

fn get_encrypted_input(client: &Client, quest: &str, seed: &str) -> (Option<String>, Option<String>, Option<String>) {

    let input_url = format!("https://everybody-codes.b-cdn.net/assets/2025/{}/input/{}.json", quest, seed);
    let input = client.get(input_url)
        .send()
        .expect("Connection (input) failed")
        .json::<serde_json::Value>()
        .expect("Deserialization (input) failed");

    let input1 = input.get("1")
        .and_then(|v| v.as_str().map(|v| v.to_string()));
    let input2 = input.get("2")
        .and_then(|v| v.as_str().map(|v| v.to_string()));
    let input3 = input.get("3")
        .and_then(|v| v.as_str().map(|v| v.to_string()));
    (input1, input2, input3)
}

fn decrypt(client: &Client, quest: &str, uuid: &str, (input1, input2, input3): (Option<String>, Option<String>, Option<String>)) -> (Option<String>, Option<String>, Option<String>) {
    let cookie = format!("everybody-codes={}", uuid);
    let url = format!("https://everybody.codes/api/event/2025/quest/{}", quest);
    let aes_json = client.get(url)
        .header("Cookie", cookie)
        .send()
        .expect("Connection (aes) failed")
        .json::<serde_json::Value>()
        .expect("Deserialization (aes) failed");

    let key1 = aes_json.get("key1")
        .and_then(|v| v.as_str());
    let key2 = aes_json.get("key2")
        .and_then(|v| v.as_str());
    let key3 = aes_json.get("key3")
        .and_then(|v| v.as_str());

    let input1 = input1
        .and_then(|input| key1
            .map(|key| decrypt_one(input, key)));
    if key2.is_none() {
        return (input1, None, None)
    }
    let input2 = input2
        .and_then(|input| key2
            .map(|key| decrypt_one(input, key)));
    if key3.is_none() {
        return (input1, input2, None)
    }
    let input3 = input3
        .and_then(|input| key3
            .map(|key| decrypt_one(input, key)));

    (input1, input2, input3)
}

fn decrypt_one(input: String, key: &str) -> String {
    let bytes = hex::decode(input)
        .expect("Decoding failed");

    let key = key.as_bytes();
    let iv_bytes = &key[..16];
    let decryptor = Decryptor::<Aes256>::new_from_slices(&key, &iv_bytes)
        .unwrap();

    let mut buf = bytes.clone();
    let plaintext = decryptor
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .expect("Decryption failed");

    String::from_utf8_lossy(plaintext).to_string()
}

pub fn get_input(quest: &str, uuid: &str) -> (Option<String>, Option<String>, Option<String>) {
    let client = Client::new();
    let seed = connect(&client, uuid);
    let input = get_encrypted_input(&client, quest, &seed);
    decrypt(&client, quest, uuid, input)
}