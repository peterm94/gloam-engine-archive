import("../pkg/index.js").then(gloam => {

    class Banana {
        update() {
            console.log("how does it know?");
        }
    }

    let ban = new Banana();
    gloam.run_update(ban);

    let script = gloam.Script.new(() => {
        console.log("banana");
    })
    script.call();
}).catch(console.error);
