var keyPair = Exonum.keyPair();
console.info("Generated Keypair:", keyPair);

var txCreate = Exonum.newMessage({
    protocol_version: 0,
    service_id: 1,
    message_id: 0,
    fields: [
        { name: 'owner', type: Exonum.PublicKey },
    ]
});

var txOrder = Exonum.newMessage({
    protocol_version: 0,
    service_id: 1,
    message_id: 1,
    fields: [
        { name: 'owner', type: Exonum.PublicKey },
        { name: 'price', type: Exonum.Uint32 },
        { name: 'amount', type: Exonum.Int32 },
        { name: 'id', type: Exonum.Uint32 },
    ]
});

var txCancel = Exonum.newMessage({
    protocol_version: 0,
    service_id: 1,
    message_id: 2,
    fields: [
        { name: 'owner', type: Exonum.PublicKey },
        { name: 'id', type: Exonum.Uint32 },
    ]
});

function sendTransaction(decl, url, data) {

    var secretKey = keyPair.secretKey;
    var signature = decl.sign(secretKey, data);

    var transaction = {
        body: data,
        network_id: decl.network_id,
        protocol_version: decl.protocol_version,
        service_id: decl.service_id,
        message_id: decl.message_id,
        signature: signature,
    };
    var body = JSON.stringify(transaction);
    console.log("URL: ", url, data, body);
    fetch(url, {
        method: 'POST',
        headers: {
            'content-type': 'application/json',
        },
        body: body,
    }).then(function(response) {
        console.log(response);
    });
}

var createAccount = function() {
    var url = "http://localhost:8080/api/services/cryptoexchange/v1/account";
    var data = {
        owner: keyPair.publicKey,
    };
    sendTransaction(txCreate, url, data);
};

var putOrder = function(price, amount, id) {
    var url = "http://localhost:8080/api/services/cryptoexchange/v1/order";
    var data = {
        owner: keyPair.publicKey,
        price: price,
        amount: amount,
        id: id,
    };
    sendTransaction(txOrder, url, data);
};

var cancelOrder = function(id) {
    var url = "http://localhost:8080/api/services/cryptoexchange/v1/cancel";
    var data = {
        owner: keyPair.publicKey,
        id: id,
    };
    sendTransaction(txCancel, url, data);
};
