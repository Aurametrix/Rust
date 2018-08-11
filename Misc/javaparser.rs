java! {
    public class Value {
        int val;

        ---

        public Value(this, int val) {
            (this.val) = val;
        }

        public void print(this) {
            System.out.println((this.val));
        }
    }
}
