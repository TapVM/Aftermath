class TopLevelClass {
    static class SubClass {
        public static int add(int y, int z) {
            return y + z;
        }
    }

    public static void main(String[] args) {
        System.out.println(SubClass.add(5, 10));
    }
}
