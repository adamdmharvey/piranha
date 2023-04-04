enum TestEnum {
   case one
   case two
   case three
   case four

   var v1: String {
      switch self {
      case .two, .four:
         return "Hello"
      case .three:
         return "Hi"
      }
   }
}
