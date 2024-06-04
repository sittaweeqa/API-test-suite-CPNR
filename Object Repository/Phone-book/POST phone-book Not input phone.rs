<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST phone-book Not input phone</name>
   <tag></tag>
   <elementGuidId>f66ae81f-f9ac-46c7-a685-b77d975a2eb7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: null,\n  \&quot;name\&quot;: {\n    \&quot;th\&quot;: \&quot;testing1\&quot;,\n    \&quot;en\&quot;: \&quot;testing1\&quot;\n  },\n  \&quot;phone\&quot;: null,\n  \&quot;category\&quot;: {\n    \&quot;id\&quot;: \&quot;${id_category_phonebook}\&quot;,\n    \&quot;type\&quot;: \&quot;https://d3jf4l1w8poe53.cloudfront.net/upload/project//phone-book//0c628fad-791f-4668-a892-aec43ce553f5.jpg\&quot;,\n    \&quot;createdAt\&quot;: \&quot;2022-05-10T01:41:02.259Z\&quot;,\n    \&quot;updatedAt\&quot;: \&quot;2022-05-10T01:41:02.259Z\&quot;,\n    \&quot;name\&quot;: {\n      \&quot;th\&quot;: \&quot;แจ้งเหตุไฟไหม้ ดับเพลิง\&quot;,\n      \&quot;en\&quot;: \&quot;Fire Department\&quot;\n    }\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>285d135f-3085-4161-b7ee-7be4b297a307</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>d77e284e-7922-437e-8fd3-90ef4fab8a82</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/project/${id_project}/phone-book</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>0ae3da21-5af3-4920-b4f6-cb1eb4b9c175</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_project</defaultValue>
      <description></description>
      <id>9d9b3720-81bd-4278-ae0a-e1e67d3c1264</id>
      <masked>false</masked>
      <name>id_project</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_category_phonebook</defaultValue>
      <description></description>
      <id>3389a480-446a-4511-8c1f-503698a4cee3</id>
      <masked>false</masked>
      <name>id_category_phonebook</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
