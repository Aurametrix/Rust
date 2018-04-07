jv foo(jv a)
    {
      return jv_array_append(a, jv_string("foo"));
    }
    
    
jv a = jv_array();
    a = jv_array_append(a, jv_true());
    a = jv_array_append(a, jv_false());
    a = jv_array_append(a, jv_null());
