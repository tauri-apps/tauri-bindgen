class Serializer {
    bytes;
    offset;
  
    constructor(len = 0) {
      this.bytes = new Uint8Array(len);
      this.offset = 0;
    }
  
    pushU8(num) {
      this.bytes[this.offset] = num
      this.offset += 1
    }
  
    push(bytes) {
      this.bytes.set(bytes, this.offset);
      this.offset += bytes.length;
    }

    filled() {
      return this.bytes.subarray(0, this.offset)
    }
  }
