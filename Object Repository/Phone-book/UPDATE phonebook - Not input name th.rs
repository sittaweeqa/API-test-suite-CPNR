<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UPDATE phonebook - Not input name th</name>
   <tag></tag>
   <elementGuidId>807ba9ef-a4dc-4021-aff4-bbe7926d6dbb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;${id_bookphone}\&quot;,\n  \&quot;name\&quot;: {\n    \&quot;th\&quot;: null,\n    \&quot;en\&quot;: \&quot;test02_edit\&quot;\n  },\n  \&quot;phone\&quot;: \&quot;0987654321\&quot;,\n  \&quot;category\&quot;: {\n    \&quot;translateColumn\&quot;: [\n      \&quot;name\&quot;\n    ],\n    \&quot;id\&quot;: \&quot;${id_category_phonebook}\&quot;,\n    \&quot;type\&quot;: \&quot;https://d3jf4l1w8poe53.cloudfront.net/upload/project//phone-book//0c628fad-791f-4668-a892-aec43ce553f5.jpg\&quot;,\n    \&quot;createdAt\&quot;: \&quot;2022-05-10T01:41:02.259Z\&quot;,\n    \&quot;updatedAt\&quot;: \&quot;2022-05-10T01:41:02.259Z\&quot;,\n    \&quot;deletedAt\&quot;: null,\n    \&quot;name\&quot;: {\n      \&quot;th\&quot;: \&quot;แจ้งเหตุไฟไหม้ ดับเพลิง\&quot;,\n      \&quot;en\&quot;: \&quot;Fire Department\&quot;\n    }\n  }\n}&quot;,
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
      <webElementGuid>5a321456-66df-42b8-a4c8-ec47933d6e7b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>161645fb-920e-4ed7-8513-7f749c0e2569</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
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
      <id>d6659c4d-ae24-4bb2-8a2e-f273c508b901</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_project</defaultValue>
      <description></description>
      <id>6382b169-8c8e-43ba-ac99-546d13ea62c6</id>
      <masked>false</masked>
      <name>id_project</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_category_phonebook</defaultValue>
      <description></description>
      <id>7e09ae1a-1821-429f-a8d4-b2ac339c3f74</id>
      <masked>false</masked>
      <name>id_category_phonebook</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_bookphone</defaultValue>
      <description></description>
      <id>f22d3ee4-4182-467a-a12a-6998d2336a6b</id>
      <masked>false</masked>
      <name>id_bookphone</name>
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
